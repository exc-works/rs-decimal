use super::{Decimal, NullDecimal, library_error, limits, resource_error, tick};
use wasm_bindgen::prelude::*;
fn text_budget(bytes: &[u8]) -> Result<(), &'static str> {
    if bytes.len() > limits::INPUT_BYTES {
        return Err("playground decoder input exceeds 8192 bytes");
    }
    if let Ok(text) = std::str::from_utf8(bytes) {
        limits::input(text)?;
    }
    // Invalid UTF-8 cannot become a valid decimal; core owns its diagnostic.
    Ok(())
}
fn json_budget(bytes: &[u8]) -> Result<(), &'static str> {
    if bytes.len() > limits::INPUT_BYTES {
        return Err("playground JSON input exceeds 8192 bytes");
    }
    let bytes = bytes.trim_ascii();
    if bytes.first() == Some(&b'"') {
        if let Ok(text) = serde_json::from_slice::<String>(bytes) {
            limits::input(&text)?;
        }
        // Core additionally accepts Go-style replacement for malformed Unicode;
        // its replacement character cannot form a valid decimal and cannot grow it.
    } else {
        text_budget(bytes)?;
    }
    Ok(())
}
#[wasm_bindgen]
impl Decimal {
    pub fn marshal_text(&self) -> Result<Vec<u8>, JsValue> {
        tick()?;
        Ok(self.inner.marshal_text())
    }
    pub fn unmarshal_text(&mut self, bytes: &[u8]) -> Result<(), JsValue> {
        tick()?;
        text_budget(bytes).map_err(resource_error)?;
        self.inner.unmarshal_text(bytes).map_err(library_error)
    }
    pub fn marshal_json(&self) -> Result<Vec<u8>, JsValue> {
        tick()?;
        Ok(self.inner.marshal_json())
    }
    pub fn unmarshal_json(&mut self, bytes: &[u8]) -> Result<(), JsValue> {
        tick()?;
        json_budget(bytes).map_err(resource_error)?;
        self.inner.unmarshal_json(bytes).map_err(library_error)
    }
    pub fn unmarshal_param(&mut self, param: &str) -> Result<(), JsValue> {
        tick()?;
        text_budget(param.as_bytes()).map_err(resource_error)?;
        self.inner.unmarshal_param(param).map_err(library_error)
    }
}
#[wasm_bindgen]
impl NullDecimal {
    pub fn marshal_text(&self) -> Result<Vec<u8>, JsValue> {
        tick()?;
        Ok(self.inner.marshal_text())
    }
    pub fn unmarshal_text(&mut self, bytes: &[u8]) -> Result<(), JsValue> {
        tick()?;
        text_budget(bytes).map_err(resource_error)?;
        self.inner.unmarshal_text(bytes).map_err(library_error)
    }
    pub fn marshal_json(&self) -> Result<Vec<u8>, JsValue> {
        tick()?;
        Ok(self.inner.marshal_json())
    }
    pub fn unmarshal_json(&mut self, bytes: &[u8]) -> Result<(), JsValue> {
        tick()?;
        json_budget(bytes).map_err(resource_error)?;
        self.inner.unmarshal_json(bytes).map_err(library_error)
    }
    pub fn unmarshal_param(&mut self, param: &str) -> Result<(), JsValue> {
        tick()?;
        text_budget(param.as_bytes()).map_err(resource_error)?;
        self.inner.unmarshal_param(param).map_err(library_error)
    }
    pub fn string(&self) -> Result<String, JsValue> {
        tick()?;
        Ok(self.inner.string())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn escaped_json_cannot_bypass_decimal_growth_preflight() {
        assert!(json_budget(br#""\u0031e100000""#).is_err());
        assert!(json_budget(b"1e-100000").is_err());
        assert!(json_budget(b"null").is_ok());
        assert!(json_budget(br#""\ud800""#).is_ok());
        assert!(text_budget(&[255]).is_ok());
        assert!(text_budget(&vec![b'1'; 8193]).is_err());
    }
}
