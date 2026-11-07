//! Framework-neutral, exact decimal rules and localized error messages.
use crate::{Decimal, DecimalError, ErrorKind};
use std::collections::BTreeMap;
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

/// Fresh owned template map; unknown locales fall back to English.
pub fn default_validation_messages(locale: &str) -> BTreeMap<String, String> {
    let locale = locale.to_lowercase().replace('-', "_");
    let matches = |language: &str| {
        locale == language
            || locale
                .strip_prefix(language)
                .is_some_and(|tail| tail.starts_with('_'))
    };
    let templates: &[(&str, &str)] = if matches("zh_hant") || matches("zh_tw") {
        &[
            ("decimal_required", "{0}為必填項"),
            ("decimal_eq", "{0}必須等於{1}"),
            ("decimal_ne", "{0}不得等於{1}"),
            ("decimal_gt", "{0}必須大於{1}"),
            ("decimal_gte", "{0}必須大於或等於{1}"),
            ("decimal_lt", "{0}必須小於{1}"),
            ("decimal_lte", "{0}必須小於或等於{1}"),
            ("decimal_between", "{0}必須介於{1}之間"),
            ("decimal_positive", "{0}必須為正數"),
            ("decimal_negative", "{0}必須為負數"),
            ("decimal_nonzero", "{0}不得為零"),
            ("decimal_max_precision", "{0}最多只能有{1}位小數"),
        ]
    } else if matches("zh") {
        &[
            ("decimal_required", "{0}为必填项"),
            ("decimal_eq", "{0}必须等于{1}"),
            ("decimal_ne", "{0}不得等于{1}"),
            ("decimal_gt", "{0}必须大于{1}"),
            ("decimal_gte", "{0}必须大于或等于{1}"),
            ("decimal_lt", "{0}必须小于{1}"),
            ("decimal_lte", "{0}必须小于或等于{1}"),
            ("decimal_between", "{0}必须介于{1}之间"),
            ("decimal_positive", "{0}必须为正数"),
            ("decimal_negative", "{0}必须为负数"),
            ("decimal_nonzero", "{0}不得为零"),
            ("decimal_max_precision", "{0}最多只能有{1}位小数"),
        ]
    } else if matches("ja") {
        &[
            ("decimal_required", "{0}は必須です"),
            ("decimal_eq", "{0}は{1}と等しくなければなりません"),
            ("decimal_ne", "{0}は{1}と等しくてはなりません"),
            ("decimal_gt", "{0}は{1}より大きくなければなりません"),
            ("decimal_gte", "{0}は{1}以上でなければなりません"),
            ("decimal_lt", "{0}は{1}より小さくなければなりません"),
            ("decimal_lte", "{0}は{1}以下でなければなりません"),
            ("decimal_between", "{0}は{1}の範囲内でなければなりません"),
            ("decimal_positive", "{0}は正の数でなければなりません"),
            ("decimal_negative", "{0}は負の数でなければなりません"),
            ("decimal_nonzero", "{0}はゼロであってはなりません"),
            (
                "decimal_max_precision",
                "{0}の小数点以下は最大{1}桁までです",
            ),
        ]
    } else if matches("ko") {
        &[
            ("decimal_required", "{0}은(는) 필수 항목입니다"),
            ("decimal_eq", "{0}은(는) {1}와(과) 같아야 합니다"),
            ("decimal_ne", "{0}은(는) {1}와(과) 같지 않아야 합니다"),
            ("decimal_gt", "{0}은(는) {1}보다 커야 합니다"),
            ("decimal_gte", "{0}은(는) {1} 이상이어야 합니다"),
            ("decimal_lt", "{0}은(는) {1}보다 작아야 합니다"),
            ("decimal_lte", "{0}은(는) {1} 이하여야 합니다"),
            ("decimal_between", "{0}은(는) {1} 사이여야 합니다"),
            ("decimal_positive", "{0}은(는) 양수여야 합니다"),
            ("decimal_negative", "{0}은(는) 음수여야 합니다"),
            ("decimal_nonzero", "{0}은(는) 0이 아니어야 합니다"),
            (
                "decimal_max_precision",
                "{0}의 소수점 이하 자릿수는 최대 {1}자리여야 합니다",
            ),
        ]
    } else if matches("fr") {
        &[
            ("decimal_required", "{0} est obligatoire"),
            ("decimal_eq", "{0} doit être égal à {1}"),
            ("decimal_ne", "{0} ne doit pas être égal à {1}"),
            ("decimal_gt", "{0} doit être supérieur à {1}"),
            ("decimal_gte", "{0} doit être supérieur ou égal à {1}"),
            ("decimal_lt", "{0} doit être inférieur à {1}"),
            ("decimal_lte", "{0} doit être inférieur ou égal à {1}"),
            ("decimal_between", "{0} doit être compris entre {1}"),
            ("decimal_positive", "{0} doit être positif"),
            ("decimal_negative", "{0} doit être négatif"),
            ("decimal_nonzero", "{0} ne doit pas être nul"),
            (
                "decimal_max_precision",
                "{0} doit avoir au plus {1} décimales",
            ),
        ]
    } else if matches("es") {
        &[
            ("decimal_required", "{0} es obligatorio"),
            ("decimal_eq", "{0} debe ser igual a {1}"),
            ("decimal_ne", "{0} no debe ser igual a {1}"),
            ("decimal_gt", "{0} debe ser mayor que {1}"),
            ("decimal_gte", "{0} debe ser mayor o igual que {1}"),
            ("decimal_lt", "{0} debe ser menor que {1}"),
            ("decimal_lte", "{0} debe ser menor o igual que {1}"),
            ("decimal_between", "{0} debe estar entre {1}"),
            ("decimal_positive", "{0} debe ser positivo"),
            ("decimal_negative", "{0} debe ser negativo"),
            ("decimal_nonzero", "{0} no debe ser cero"),
            (
                "decimal_max_precision",
                "{0} debe tener como máximo {1} decimales",
            ),
        ]
    } else if matches("de") {
        &[
            ("decimal_required", "{0} ist erforderlich"),
            ("decimal_eq", "{0} muss gleich {1} sein"),
            ("decimal_ne", "{0} darf nicht gleich {1} sein"),
            ("decimal_gt", "{0} muss größer als {1} sein"),
            ("decimal_gte", "{0} muss größer oder gleich {1} sein"),
            ("decimal_lt", "{0} muss kleiner als {1} sein"),
            ("decimal_lte", "{0} muss kleiner oder gleich {1} sein"),
            ("decimal_between", "{0} muss zwischen {1} liegen"),
            ("decimal_positive", "{0} muss positiv sein"),
            ("decimal_negative", "{0} muss negativ sein"),
            ("decimal_nonzero", "{0} darf nicht null sein"),
            (
                "decimal_max_precision",
                "{0} darf höchstens {1} Dezimalstellen haben",
            ),
        ]
    } else if matches("pt") {
        &[
            ("decimal_required", "{0} é obrigatório"),
            ("decimal_eq", "{0} deve ser igual a {1}"),
            ("decimal_ne", "{0} não deve ser igual a {1}"),
            ("decimal_gt", "{0} deve ser maior que {1}"),
            ("decimal_gte", "{0} deve ser maior ou igual a {1}"),
            ("decimal_lt", "{0} deve ser menor que {1}"),
            ("decimal_lte", "{0} deve ser menor ou igual a {1}"),
            ("decimal_between", "{0} deve estar entre {1}"),
            ("decimal_positive", "{0} deve ser positivo"),
            ("decimal_negative", "{0} deve ser negativo"),
            ("decimal_nonzero", "{0} não deve ser zero"),
            (
                "decimal_max_precision",
                "{0} deve ter no máximo {1} casas decimais",
            ),
        ]
    } else if matches("ru") {
        &[
            ("decimal_required", "{0} обязательно для заполнения"),
            ("decimal_eq", "{0} должно быть равно {1}"),
            ("decimal_ne", "{0} не должно быть равно {1}"),
            ("decimal_gt", "{0} должно быть больше {1}"),
            ("decimal_gte", "{0} должно быть больше или равно {1}"),
            ("decimal_lt", "{0} должно быть меньше {1}"),
            ("decimal_lte", "{0} должно быть меньше или равно {1}"),
            ("decimal_between", "{0} должно быть в диапазоне {1}"),
            ("decimal_positive", "{0} должно быть положительным"),
            ("decimal_negative", "{0} должно быть отрицательным"),
            ("decimal_nonzero", "{0} не должно быть равно нулю"),
            (
                "decimal_max_precision",
                "{0} должно иметь не более {1} знаков после запятой",
            ),
        ]
    } else if matches("ar") {
        &[
            ("decimal_required", "{0} مطلوب"),
            ("decimal_eq", "يجب أن يكون {0} مساوياً لـ {1}"),
            ("decimal_ne", "يجب ألا يكون {0} مساوياً لـ {1}"),
            ("decimal_gt", "يجب أن يكون {0} أكبر من {1}"),
            ("decimal_gte", "يجب أن يكون {0} أكبر من أو يساوي {1}"),
            ("decimal_lt", "يجب أن يكون {0} أصغر من {1}"),
            ("decimal_lte", "يجب أن يكون {0} أصغر من أو يساوي {1}"),
            ("decimal_between", "يجب أن يكون {0} بين {1}"),
            ("decimal_positive", "يجب أن يكون {0} موجباً"),
            ("decimal_negative", "يجب أن يكون {0} سالباً"),
            ("decimal_nonzero", "يجب ألا يكون {0} صفراً"),
            (
                "decimal_max_precision",
                "يجب ألا يحتوي {0} على أكثر من {1} منزلة عشرية",
            ),
        ]
    } else if matches("hi") {
        &[
            ("decimal_required", "{0} आवश्यक है"),
            ("decimal_eq", "{0} {1} के बराबर होना चाहिए"),
            ("decimal_ne", "{0} {1} के बराबर नहीं होना चाहिए"),
            ("decimal_gt", "{0} {1} से अधिक होना चाहिए"),
            ("decimal_gte", "{0} {1} से अधिक या बराबर होना चाहिए"),
            ("decimal_lt", "{0} {1} से कम होना चाहिए"),
            ("decimal_lte", "{0} {1} से कम या बराबर होना चाहिए"),
            ("decimal_between", "{0} {1} के बीच होना चाहिए"),
            ("decimal_positive", "{0} धनात्मक होना चाहिए"),
            ("decimal_negative", "{0} ऋणात्मक होना चाहिए"),
            ("decimal_nonzero", "{0} शून्य नहीं होना चाहिए"),
            (
                "decimal_max_precision",
                "{0} में अधिकतम {1} दशमलव स्थान होने चाहिए",
            ),
        ]
    } else {
        &[
            ("decimal_required", "{0} is required"),
            ("decimal_eq", "{0} must be equal to {1}"),
            ("decimal_ne", "{0} must not be equal to {1}"),
            ("decimal_gt", "{0} must be greater than {1}"),
            ("decimal_gte", "{0} must be greater than or equal to {1}"),
            ("decimal_lt", "{0} must be less than {1}"),
            ("decimal_lte", "{0} must be less than or equal to {1}"),
            ("decimal_between", "{0} must be between {1}"),
            ("decimal_positive", "{0} must be positive"),
            ("decimal_negative", "{0} must be negative"),
            ("decimal_nonzero", "{0} must not be zero"),
            (
                "decimal_max_precision",
                "{0} must have at most {1} decimal places",
            ),
        ]
    };
    templates
        .iter()
        .map(|(tag, text)| (tag.to_string(), text.to_string()))
        .collect()
}
