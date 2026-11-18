use super::{
    Decimal, NullDecimal, exception, library_error, limits, parameter_error, resource_error,
    text_json, tick,
};
use rs_decimal::{config as core, config::ConfigValue};
use wasm_bindgen::prelude::*;
fn value<'a>(
    kind: &str,
    text: &'a str,
    number: f64,
    bytes: &'a [u8],
) -> Result<ConfigValue<'a>, JsValue> {
    Ok(match kind {
        "Null" => ConfigValue::Null,
        "Unsupported" => ConfigValue::Unsupported,
        "Text" | "Number" => {
            limits::input(text).map_err(resource_error)?;
            if kind == "Text" {
                ConfigValue::Text(text)
            } else {
                ConfigValue::Number(text)
            }
        }
        "Bytes" => {
            text_json::text_budget(bytes).map_err(resource_error)?;
            ConfigValue::Bytes(bytes)
        }
        "Signed" => {
            if text.len() > 40 {
                return Err(parameter_error("value must fit i128"));
            }
            ConfigValue::Signed(
                text.parse()
                    .map_err(|_| parameter_error("value must fit i128"))?,
            )
        }
        "Unsigned" => {
            if text.len() > 39 {
                return Err(parameter_error("value must fit u128"));
            }
            ConfigValue::Unsigned(
                text.parse()
                    .map_err(|_| parameter_error("value must fit u128"))?,
            )
        }
        "Float32" => ConfigValue::Float32(number as f32),
        "Float64" => ConfigValue::Float64(number),
        _ => return Err(parameter_error("expected a primitive ConfigValue variant")),
    })
}
#[wasm_bindgen]
pub fn config_decode_decimal(
    kind: &str,
    text: &str,
    number: f64,
    bytes: &[u8],
) -> Result<Decimal, JsValue> {
    tick()?;
    core::decode_decimal(value(kind, text, number, bytes)?)
        .map(|inner| Decimal { inner })
        .map_err(library_error)
}
#[wasm_bindgen]
pub fn config_decode_nullable(
    kind: &str,
    text: &str,
    number: f64,
    bytes: &[u8],
) -> Result<NullDecimal, JsValue> {
    tick()?;
    core::decode_nullable(value(kind, text, number, bytes)?)
        .map(|inner| NullDecimal { inner })
        .map_err(library_error)
}
#[wasm_bindgen]
pub fn config_decimal_from_decimal(value: &Decimal) -> Result<Decimal, JsValue> {
    tick()?;
    core::decode_decimal(ConfigValue::Decimal(&value.inner))
        .map(|inner| Decimal { inner })
        .map_err(library_error)
}
#[wasm_bindgen]
pub fn config_decimal_from_nullable(value: &NullDecimal) -> Result<Decimal, JsValue> {
    tick()?;
    core::decode_decimal(ConfigValue::Nullable(&value.inner))
        .map(|inner| Decimal { inner })
        .map_err(library_error)
}
#[wasm_bindgen]
pub fn config_nullable_from_decimal(value: &Decimal) -> Result<NullDecimal, JsValue> {
    tick()?;
    core::decode_nullable(ConfigValue::Decimal(&value.inner))
        .map(|inner| NullDecimal { inner })
        .map_err(library_error)
}
#[wasm_bindgen]
pub fn config_nullable_from_nullable(value: &NullDecimal) -> Result<NullDecimal, JsValue> {
    tick()?;
    core::decode_nullable(ConfigValue::Nullable(&value.inner))
        .map(|inner| NullDecimal { inner })
        .map_err(library_error)
}
fn serde_error(error: serde_json::Error) -> JsValue {
    exception("serde", &error.to_string(), &[])
}
#[wasm_bindgen]
impl Decimal {
    pub fn serialize(&self) -> Result<String, JsValue> {
        tick()?;
        serde_json::to_string(&self.inner).map_err(serde_error)
    }
    pub fn deserialize(source: &str) -> Result<Decimal, JsValue> {
        tick()?;
        text_json::json_budget(source.as_bytes()).map_err(resource_error)?;
        serde_json::from_str(source)
            .map(|inner| Self { inner })
            .map_err(serde_error)
    }
}
#[wasm_bindgen]
impl NullDecimal {
    pub fn serialize(&self) -> Result<String, JsValue> {
        tick()?;
        serde_json::to_string(&self.inner).map_err(serde_error)
    }
    pub fn deserialize(source: &str) -> Result<NullDecimal, JsValue> {
        tick()?;
        text_json::json_budget(source.as_bytes()).map_err(resource_error)?;
        serde_json::from_str(source)
            .map(|inner| Self { inner })
            .map_err(serde_error)
    }
}
#[wasm_bindgen]
pub fn config_deserialize_json_decimal(source: &str) -> Result<Decimal, JsValue> {
    tick()?;
    text_json::json_budget(source.as_bytes()).map_err(resource_error)?;
    let mut deserializer = serde_json::Deserializer::from_str(source);
    let inner = core::deserialize_json_decimal(&mut deserializer).map_err(serde_error)?;
    deserializer.end().map_err(serde_error)?;
    Ok(Decimal { inner })
}
#[wasm_bindgen]
pub fn config_deserialize_json_nullable(source: &str) -> Result<NullDecimal, JsValue> {
    tick()?;
    text_json::json_budget(source.as_bytes()).map_err(resource_error)?;
    let mut deserializer = serde_json::Deserializer::from_str(source);
    let inner = core::deserialize_json_nullable(&mut deserializer).map_err(serde_error)?;
    deserializer.end().map_err(serde_error)?;
    Ok(NullDecimal { inner })
}
