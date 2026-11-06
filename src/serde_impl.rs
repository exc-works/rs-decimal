//! Generic Serde uses exact strings (or exact integer visitor inputs) and null.
//! Floating-point visitor inputs are rejected; use the json feature for raw numbers.
use crate::{Decimal, NullDecimal};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};
use std::fmt;
impl Serialize for Decimal {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if self.is_nil() {
            s.serialize_none()
        } else {
            s.serialize_str(&self.string())
        }
    }
}
impl Serialize for NullDecimal {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if self.valid {
            s.serialize_str(&self.decimal.string())
        } else {
            s.serialize_none()
        }
    }
}
struct DecimalVisitor;
impl Visitor<'_> for DecimalVisitor {
    type Value = Option<Decimal>;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            "an exact decimal string, integer, or null; use unmarshal_json for raw JSON numbers",
        )
    }
    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
    fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
        Decimal::new_from_string(s).map(Some).map_err(E::custom)
    }
    fn visit_i64<E: de::Error>(self, n: i64) -> Result<Self::Value, E> {
        Ok(Some(Decimal::new(n)))
    }
    fn visit_u64<E: de::Error>(self, n: u64) -> Result<Self::Value, E> {
        Ok(Some(Decimal::new_from_uint64(n, 0)))
    }
    fn visit_i128<E: de::Error>(self, n: i128) -> Result<Self::Value, E> {
        self.visit_str(&n.to_string())
    }
    fn visit_u128<E: de::Error>(self, n: u128) -> Result<Self::Value, E> {
        self.visit_str(&n.to_string())
    }
}
impl<'de> Deserialize<'de> for Decimal {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(d.deserialize_any(DecimalVisitor)?.unwrap_or_default())
    }
}
impl<'de> Deserialize<'de> for NullDecimal {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(d.deserialize_any(DecimalVisitor)?
            .map(Self::new)
            .unwrap_or_default())
    }
}
