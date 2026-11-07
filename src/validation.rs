//! Framework-neutral, exact decimal rules and localized error messages.
use crate::{Decimal, DecimalError, ErrorKind};
use std::collections::BTreeMap;

mod messages;
pub use messages::default_validation_messages;
#[derive(Debug, Clone)]
enum Kind {
    Required,
    Eq(Decimal),
    Ne(Decimal),
    Gt(Decimal),
    Gte(Decimal),
    Lt(Decimal),
    Lte(Decimal),
    Between(Decimal, Decimal),
    Positive,
    Negative,
    Nonzero,
    MaxScale(usize),
}
/// A validated rule. Invalid rule parameters return an error at construction time.
#[derive(Debug, Clone)]
pub struct ValidationRule {
    kind: Kind,
    tag: &'static str,
    parameter: String,
}
/// One failed rule, retaining the field name and rule parameter for translation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// Caller-supplied field name.
    pub field: String,
    /// The validated rule tag.
    pub tag: &'static str,
    /// Original rule parameter text.
    pub parameter: String,
}
fn invalid() -> DecimalError {
    DecimalError::new(
        ErrorKind::InvalidArgument,
        "invalid decimal validation rule or message",
    )
}
impl ValidationRule {
    /// Parses a `decimal_*` rule and validates its parameter.
    /// Comparison rules take decimal text; `decimal_between` takes `lower~upper`;
    /// `decimal_max_precision` takes a nonnegative scale. Unknown tags, invalid
    /// parameters and reversed ranges return `InvalidArgument`.
    pub fn parse(tag: &str, parameter: &str) -> Result<Self, DecimalError> {
        let decimal = || Decimal::new_from_string(parameter).map_err(|_| invalid());
        let (tag, kind) = match tag {
            "decimal_required" => ("decimal_required", Kind::Required),
            "decimal_eq" => ("decimal_eq", Kind::Eq(decimal()?)),
            "decimal_ne" => ("decimal_ne", Kind::Ne(decimal()?)),
            "decimal_gt" => ("decimal_gt", Kind::Gt(decimal()?)),
            "decimal_gte" => ("decimal_gte", Kind::Gte(decimal()?)),
            "decimal_lt" => ("decimal_lt", Kind::Lt(decimal()?)),
            "decimal_lte" => ("decimal_lte", Kind::Lte(decimal()?)),
            "decimal_between" => {
                let (lower, upper) = parameter.trim().split_once('~').ok_or_else(invalid)?;
                let lower = Decimal::new_from_string(lower).map_err(|_| invalid())?;
                let upper = Decimal::new_from_string(upper).map_err(|_| invalid())?;
                if lower > upper {
                    return Err(invalid());
                }
                ("decimal_between", Kind::Between(lower, upper))
            }
            "decimal_positive" => ("decimal_positive", Kind::Positive),
            "decimal_negative" => ("decimal_negative", Kind::Negative),
            "decimal_nonzero" => ("decimal_nonzero", Kind::Nonzero),
            "decimal_max_precision" => {
                let max = parameter.trim().parse::<isize>().map_err(|_| invalid())?;
                if max < 0 {
                    return Err(invalid());
                }
                ("decimal_max_precision", Kind::MaxScale(max as usize))
            }
            _ => return Err(invalid()),
        };
        Ok(Self {
            kind,
            tag,
            parameter: parameter.to_owned(),
        })
    }
    /// Evaluates the exact rule without a float conversion.
    /// `decimal_required` checks initialization, so initialized zero is present.
    /// Comparison and max-precision rules alone do not require initialization.
    pub fn matches(&self, value: &Decimal) -> bool {
        match &self.kind {
            Kind::Required => !value.is_nil(),
            Kind::Eq(d) => value == d,
            Kind::Ne(d) => value != d,
            Kind::Gt(d) => value > d,
            Kind::Gte(d) => value >= d,
            Kind::Lt(d) => value < d,
            Kind::Lte(d) => value <= d,
            Kind::Between(lower, upper) => !value.is_nil() && value >= lower && value <= upper,
            Kind::Positive => !value.is_nil() && value.is_positive(),
            Kind::Negative => !value.is_nil() && value.is_negative(),
            Kind::Nonzero => !value.is_nil() && !value.is_zero(),
            Kind::MaxScale(max) => value.scale <= *max,
        }
    }
    /// Returns the field, tag and parameter when the value fails this rule.
    pub fn validate(&self, field: &str, value: &Decimal) -> Result<(), ValidationError> {
        if self.matches(value) {
            Ok(())
        } else {
            Err(ValidationError {
                field: field.to_owned(),
                tag: self.tag,
                parameter: self.parameter.clone(),
            })
        }
    }
}
/// Collects every failing rule in caller order; no reflection or global registry.
pub fn validate_decimal(
    field: &str,
    value: &Decimal,
    rules: &[ValidationRule],
) -> Vec<ValidationError> {
    rules
        .iter()
        .filter_map(|r| r.validate(field, value).err())
        .collect()
}
/// Owned message templates with single-pass placeholder substitution.
#[derive(Debug, Clone)]
pub struct ValidationMessages {
    templates: BTreeMap<String, String>,
}
impl ValidationMessages {
    /// Loads message templates for a locale, falling back to English for unknown locales.
    pub fn new(locale: &str) -> Self {
        Self {
            templates: default_validation_messages(locale),
        }
    }
    /// Overrides English defaults. Unknown tags and unsupported placeholders are rejected.
    pub fn with_messages(messages: BTreeMap<String, String>) -> Result<Self, DecimalError> {
        let mut out = Self::new("en");
        for (tag, text) in messages {
            if !out.templates.contains_key(&tag) || text.is_empty() {
                return Err(invalid());
            }
            let mut remainder = text.as_str();
            while let Some(i) = remainder.find(['{', '}']) {
                let rest = &remainder[i..];
                if rest.starts_with("{0}") || rest.starts_with("{1}") {
                    remainder = &rest[3..];
                } else {
                    return Err(invalid());
                }
            }
            out.templates.insert(tag, text);
        }
        Ok(out)
    }
    /// Formats failures in order, substituting field `{0}` and parameter `{1}` once.
    /// Inserted user text is never reinterpreted as a template.
    pub fn translate(&self, errors: &[ValidationError]) -> Vec<String> {
        errors
            .iter()
            .map(|e| {
                let mut template = self
                    .templates
                    .get(e.tag)
                    .map_or("{0} is invalid", String::as_str);
                let mut out = String::new();
                while let Some(i) = template.find('{') {
                    out.push_str(&template[..i]);
                    let rest = &template[i..];
                    if rest.starts_with("{0}") {
                        out.push_str(&e.field);
                    } else {
                        out.push_str(&e.parameter);
                    }
                    template = &rest[3..];
                }
                out.push_str(template);
                out
            })
            .collect()
    }
}
