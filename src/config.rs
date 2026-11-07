//! Explicit configuration conversions and JSON-specific Serde field adapters.
use crate::{BigInt, Decimal, DecimalError, ErrorKind, NullDecimal};
use serde::{Deserialize, Deserializer, de::Error};

/// Typed configuration inputs. JSON field adapters never pass numbers through floats.
#[derive(Debug, Clone, Copy)]
pub enum ConfigValue<'a> {
    /// An absent value.
    Null,
    /// Borrowed decimal text.
    Text(&'a str),
    /// Borrowed bytes containing decimal text.
    Bytes(&'a [u8]),
    /// An exact numeric token supplied as text.
    Number(&'a str),
    /// A signed integer with no floating-point conversion.
    Signed(i128),
    /// An unsigned integer with no floating-point conversion.
    Unsigned(u128),
    /// A binary32 value; nonfinite values are rejected.
    Float32(f32),
    /// A binary64 value; nonfinite values are rejected.
    Float64(f64),
    /// An existing decimal, preserving representation when copied.
    Decimal(&'a Decimal),
    /// An existing nullable decimal, including its validity state.
    Nullable(&'a NullDecimal),
    /// An unsupported source type, rejected without coercion.
    Unsupported,
}
fn invalid() -> DecimalError {
    DecimalError::new(
        ErrorKind::Unmarshal,
        "unsupported or null decimal configuration value",
    )
}
fn number(text: &str) -> Result<Decimal, DecimalError> {
    Decimal::new_from_string(text).map_err(|e| e.with_kind(ErrorKind::Unmarshal))
}
/// Scalar equivalent of Go DecodeHook for a non-nullable Decimal target.
pub fn decode_decimal(value: ConfigValue<'_>) -> Result<Decimal, DecimalError> {
    match value {
        ConfigValue::Text(s) | ConfigValue::Number(s) => number(s),
        ConfigValue::Bytes(s) => number(&String::from_utf8_lossy(s)),
        ConfigValue::Signed(n) => Ok(Decimal::new_from_big_int(&BigInt::from(n))),
        ConfigValue::Unsigned(n) => Ok(Decimal::new_from_big_int(&BigInt::from(n))),
        ConfigValue::Float32(f) if f.is_finite() => Ok(Decimal::new_from_float32(f)),
        ConfigValue::Float64(f) if f.is_finite() => Ok(Decimal::new_from_float64(f)),
        ConfigValue::Decimal(d) => Ok(d.clone()),
        ConfigValue::Nullable(n) if n.valid => Ok(n.decimal.clone()),
        _ => Err(invalid()),
    }
}
/// Decodes a nullable field; null and exactly empty text/bytes become invalid.
/// Other inputs follow [`decode_decimal`]; existing nullable values retain their state.
pub fn decode_nullable(value: ConfigValue<'_>) -> Result<NullDecimal, DecimalError> {
    match value {
        ConfigValue::Null | ConfigValue::Text("") | ConfigValue::Bytes([]) => {
            Ok(NullDecimal::default())
        }
        ConfigValue::Nullable(n) => Ok(n.clone()),
        other => decode_decimal(other).map(NullDecimal::new),
    }
}
fn json_decimal(raw: &str) -> Result<Decimal, DecimalError> {
    if raw == "null" {
        return Err(invalid());
    }
    let mut value = Decimal::default();
    value.unmarshal_json(raw.as_bytes())?;
    Ok(value)
}
/// Use with `#[serde(deserialize_with = "rs_decimal::config::deserialize_json_decimal")]`.
/// Requires a serde_json source deserializer; retains the original numeric token.
pub fn deserialize_json_decimal<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Decimal, D::Error> {
    let raw = Box::<serde_json::value::RawValue>::deserialize(deserializer)?;
    json_decimal(raw.get()).map_err(D::Error::custom)
}
/// JSON null and an empty JSON string create an invalid NullDecimal.
pub fn deserialize_json_nullable<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<NullDecimal, D::Error> {
    let raw = Box::<serde_json::value::RawValue>::deserialize(deserializer)?;
    if raw.get() == "null" || raw.get() == "\"\"" {
        return Ok(NullDecimal::default());
    }
    json_decimal(raw.get())
        .map(NullDecimal::new)
        .map_err(D::Error::custom)
}
