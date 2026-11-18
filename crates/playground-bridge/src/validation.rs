use super::{Decimal, library_error, limits, parameter_error, resource_error, tick};
use rs_decimal::validation::{self as core, ValidationError};
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;
const ITEMS: usize = 64;
const TAGS: [&str; 12] = [
    "decimal_required",
    "decimal_eq",
    "decimal_ne",
    "decimal_gt",
    "decimal_gte",
    "decimal_lt",
    "decimal_lte",
    "decimal_between",
    "decimal_positive",
    "decimal_negative",
    "decimal_nonzero",
    "decimal_max_precision",
];
fn text(value: &str) -> Result<(), JsValue> {
    if value.len() > limits::INPUT_BYTES {
        return Err(resource_error(
            "playground validation string exceeds 8192 bytes",
        ));
    }
    Ok(())
}
fn json<T: serde::Serialize>(value: &T) -> Result<String, JsValue> {
    let encoded = serde_json::to_string(value).unwrap();
    limits::buffer(encoded.len()).map_err(resource_error)?;
    Ok(encoded)
}
fn parse<T: serde::de::DeserializeOwned>(source: &str) -> Result<T, JsValue> {
    limits::buffer(source.len()).map_err(resource_error)?;
    serde_json::from_str(source).map_err(|_| parameter_error("invalid validation bridge data"))
}
fn rule(tag: &str, parameter: &str) -> Result<core::ValidationRule, JsValue> {
    text(tag)?;
    text(parameter)?;
    match tag {
        "decimal_eq" | "decimal_ne" | "decimal_gt" | "decimal_gte" | "decimal_lt"
        | "decimal_lte" => limits::input(parameter).map_err(resource_error)?,
        "decimal_between" => {
            if let Some((lower, upper)) = parameter.trim().split_once('~') {
                limits::input(lower).map_err(resource_error)?;
                limits::input(upper).map_err(resource_error)?;
            }
        }
        _ => {}
    }
    core::ValidationRule::parse(tag, parameter).map_err(library_error)
}
fn error_json(e: &ValidationError) -> serde_json::Value {
    serde_json::json!({"field":e.field,"tag":e.tag,"parameter":e.parameter})
}
#[wasm_bindgen]
pub struct ValidationRule {
    inner: core::ValidationRule,
    tag: String,
    parameter: String,
}
#[wasm_bindgen]
impl ValidationRule {
    pub fn parse(tag: &str, parameter: &str) -> Result<ValidationRule, JsValue> {
        tick()?;
        Ok(Self {
            inner: rule(tag, parameter)?,
            tag: tag.into(),
            parameter: parameter.into(),
        })
    }
    pub fn matches(&self, value: &Decimal) -> Result<bool, JsValue> {
        tick()?;
        Ok(self.inner.matches(&value.inner))
    }
    pub fn validate(&self, field: &str, value: &Decimal) -> Result<String, JsValue> {
        tick()?;
        text(field)?;
        json(
            &self
                .inner
                .validate(field, &value.inner)
                .err()
                .as_ref()
                .map(error_json),
        )
    }
    pub fn spec(&self) -> Result<String, JsValue> {
        tick()?;
        json(&[&self.tag, &self.parameter])
    }
}
#[wasm_bindgen]
pub fn validate_decimal(field: &str, value: &Decimal, specs: &str) -> Result<String, JsValue> {
    tick()?;
    text(field)?;
    let specs: Vec<[String; 2]> = parse(specs)?;
    if specs.len() > ITEMS {
        return Err(resource_error(
            "playground validation list exceeds 64 items",
        ));
    }
    let rules = specs
        .iter()
        .map(|[tag, parameter]| rule(tag, parameter))
        .collect::<Result<Vec<_>, _>>()?;
    json(
        &core::validate_decimal(field, &value.inner, &rules)
            .iter()
            .map(error_json)
            .collect::<Vec<_>>(),
    )
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ErrorInput {
    field: String,
    tag: String,
    parameter: String,
}
#[wasm_bindgen]
pub struct ValidationMessages {
    inner: core::ValidationMessages,
    templates: BTreeMap<String, String>,
}
fn output_budget(
    templates: &BTreeMap<String, String>,
    errors: &[ErrorInput],
) -> Result<(), &'static str> {
    let mut total = 0usize;
    for e in errors {
        let template = templates
            .get(&e.tag)
            .map_or("{0} is invalid", String::as_str);
        // Templates have already passed core placeholder validation. This upper
        // bound includes placeholders themselves, so it is conservative.
        total = total
            .saturating_add(template.len())
            .saturating_add(
                template
                    .matches("{0}")
                    .count()
                    .saturating_mul(e.field.len()),
            )
            .saturating_add(
                template
                    .matches("{1}")
                    .count()
                    .saturating_mul(e.parameter.len()),
            );
        limits::buffer(total)?;
    }
    Ok(())
}
#[wasm_bindgen]
impl ValidationMessages {
    pub fn new(locale: &str) -> Result<ValidationMessages, JsValue> {
        tick()?;
        text(locale)?;
        Ok(Self {
            inner: core::ValidationMessages::new(locale),
            templates: core::default_validation_messages(locale),
        })
    }
    pub fn with_messages(messages: &str) -> Result<ValidationMessages, JsValue> {
        tick()?;
        let messages: BTreeMap<String, String> = parse(messages)?;
        if messages.len() > ITEMS {
            return Err(resource_error(
                "playground validation map exceeds 64 entries",
            ));
        }
        for (tag, value) in &messages {
            text(tag)?;
            text(value)?;
        }
        let inner =
            core::ValidationMessages::with_messages(messages.clone()).map_err(library_error)?;
        let mut templates = core::default_validation_messages("en");
        templates.extend(messages);
        Ok(Self { inner, templates })
    }
    pub fn translate(&self, errors: &str) -> Result<String, JsValue> {
        tick()?;
        let errors: Vec<ErrorInput> = parse(errors)?;
        if errors.len() > ITEMS {
            return Err(resource_error(
                "playground validation list exceeds 64 items",
            ));
        }
        for e in &errors {
            text(&e.field)?;
            text(&e.tag)?;
            text(&e.parameter)?;
        }
        output_budget(&self.templates, &errors).map_err(resource_error)?;
        // Core tags are &'static str. Map recognized tags to their static names;
        // every other owned name selects the core's same unknown-tag fallback.
        let errors = errors
            .into_iter()
            .map(|e| ValidationError {
                field: e.field,
                parameter: e.parameter,
                tag: TAGS.into_iter().find(|tag| *tag == e.tag).unwrap_or(""),
            })
            .collect::<Vec<_>>();
        json(&self.inner.translate(&errors))
    }
}
#[wasm_bindgen]
pub fn default_validation_messages(locale: &str) -> Result<String, JsValue> {
    tick()?;
    text(locale)?;
    json(&core::default_validation_messages(locale))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn substitution_is_bounded_before_core_allocation() {
        let templates = BTreeMap::from([("decimal_required".into(), "{0}".repeat(2000))]);
        let e = ErrorInput {
            field: "x".repeat(8192),
            tag: "decimal_required".into(),
            parameter: String::new(),
        };
        assert!(output_budget(&templates, &[e]).is_err());
        let e = ErrorInput {
            field: "amount{1}".into(),
            tag: "unknown".into(),
            parameter: "0".into(),
        };
        assert!(output_budget(&templates, &[e]).is_ok());
    }
}
