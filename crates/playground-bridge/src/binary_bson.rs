use super::{
    Decimal, NullDecimal, checked_i32, library_error, limits, parameter_error, resource_error,
    text_json, tick,
};
use wasm_bindgen::prelude::*;
fn binary_budget(bytes: &[u8]) -> Result<(), &'static str> {
    limits::buffer(bytes.len())?;
    if bytes.len() < 4 {
        return Ok(());
    }
    let scale = u32::from_be_bytes(bytes[..4].try_into().unwrap());
    if scale > 131072 || (bytes.len() > 4 && bytes[4] >> 1 != 1) {
        return Ok(());
    } // core rejects before allocation
    if scale as usize > limits::SCALE {
        return Err("playground binary scale exceeds budget");
    }
    if let Some(magnitude) = bytes.get(5..) {
        if let Some(first) = magnitude.iter().position(|b| *b != 0) {
            let bits = (magnitude.len() - first - 1) as u64 * 8 + 8
                - magnitude[first].leading_zeros() as u64;
            if bits > limits::COEFFICIENT_BITS {
                return Err("playground binary coefficient exceeds budget");
            }
        }
    }
    Ok(())
}
fn kind(value: f64) -> Result<u8, JsValue> {
    u8::try_from(checked_i32(value).map_err(parameter_error)?)
        .map_err(|_| parameter_error("kind must fit u8"))
}
fn bson_budget(kind: u8, bytes: &[u8]) -> Result<(), &'static str> {
    limits::buffer(bytes.len())?;
    match kind {
        2 if bytes.len() >= 4 => {
            let length = i32::from_le_bytes(bytes[..4].try_into().unwrap());
            if length > 0 {
                if let Some(value) = bytes.get(4..).and_then(|b| b.get(..length as usize)) {
                    // Core deliberately skips the final byte without requiring NUL.
                    text_json::text_budget(&value[..value.len() - 1])?;
                }
            }
        }
        19 if bytes.len() >= 16 => {
            let low = u64::from_le_bytes(bytes[..8].try_into().unwrap());
            let high = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
            if matches!((high >> 58) & 31, 30 | 31) {
                return Ok(());
            }
            let (coefficient, exponent) = if (high >> 61) & 3 == 3 {
                (0u128, ((high >> 47) & 0x3fff) as i32 - 6176)
            } else {
                (
                    ((u128::from(high & ((1 << 49) - 1))) << 64) | u128::from(low),
                    ((high >> 49) & 0x3fff) as i32 - 6176,
                )
            };
            limits::input(&format!("{coefficient}e{exponent}"))?;
        }
        _ => {} // fixed numeric widths/null or core-invalid kinds cannot expand excessively
    }
    Ok(())
}
fn bson_tuple(value: (u8, Vec<u8>)) -> js_sys::Array {
    let result = js_sys::Array::new();
    result.push(&JsValue::from_f64(value.0 as f64));
    result.push(&js_sys::Uint8Array::from(value.1.as_slice()));
    result
}
#[wasm_bindgen]
impl Decimal {
    pub fn marshal_binary(&self) -> Result<Vec<u8>, JsValue> {
        tick()?;
        self.inner.marshal_binary().map_err(library_error)
    }
    pub fn unmarshal_binary(&mut self, bytes: &[u8]) -> Result<(), JsValue> {
        tick()?;
        binary_budget(bytes).map_err(resource_error)?;
        self.inner.unmarshal_binary(bytes).map_err(library_error)
    }
}
#[wasm_bindgen]
impl Decimal {
    pub fn marshal_bson_value(&self) -> Result<js_sys::Array, JsValue> {
        tick()?;
        self.inner
            .marshal_bson_value()
            .map(bson_tuple)
            .map_err(library_error)
    }
    pub fn unmarshal_bson_value(&mut self, value_kind: f64, data: &[u8]) -> Result<(), JsValue> {
        tick()?;
        let kind = kind(value_kind)?;
        bson_budget(kind, data).map_err(resource_error)?;
        self.inner
            .unmarshal_bson_value(kind, data)
            .map_err(library_error)
    }
}
#[wasm_bindgen]
impl NullDecimal {
    pub fn marshal_bson_value(&self) -> Result<js_sys::Array, JsValue> {
        tick()?;
        self.inner
            .marshal_bson_value()
            .map(bson_tuple)
            .map_err(library_error)
    }
    pub fn unmarshal_bson_value(&mut self, value_kind: f64, data: &[u8]) -> Result<(), JsValue> {
        tick()?;
        let kind = kind(value_kind)?;
        bson_budget(kind, data).map_err(resource_error)?;
        self.inner
            .unmarshal_bson_value(kind, data)
            .map_err(library_error)
    }
}
#[wasm_bindgen]
pub fn precision_fixed_size() -> Result<u32, JsValue> {
    tick()?;
    Ok(rs_decimal::PRECISION_FIXED_SIZE as u32)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binary_and_bson_preflight_checks_wire_growth_without_mutation() {
        assert!(binary_budget(&[0, 0, 3, 233, 2, 1]).is_err());
        assert!(binary_budget(&[0, 2, 0, 1, 2, 1]).is_ok()); // invalid core wire scale
        let mut large = vec![0, 0, 0, 0, 2];
        large.extend(vec![255; 2049]);
        assert!(binary_budget(&large).is_err());
        let mut decimal128 = 1u64.to_le_bytes().to_vec();
        decimal128.extend(((6176u64 + 1001) << 49).to_le_bytes());
        assert!(bson_budget(19, &decimal128).is_err());
        let mut text = 9i32.to_le_bytes().to_vec();
        text.extend(b"1e100000\0");
        assert!(bson_budget(2, &text).is_err());
    }
}
