use super::{Decimal, NullDecimal, library_error, limits, parameter_error, resource_error, tick};
use rs_decimal::{DecimalError as CoreError, ErrorKind, ScanValue};
use wasm_bindgen::prelude::*;
const KINDS: [ErrorKind; 11] = [
    ErrorKind::DivideByZero,
    ErrorKind::InvalidArgument,
    ErrorKind::InvalidFormat,
    ErrorKind::InvalidLog,
    ErrorKind::InvalidPrecision,
    ErrorKind::InvalidRoot,
    ErrorKind::NegativeRoot,
    ErrorKind::NoConvergence,
    ErrorKind::Overflow,
    ErrorKind::RoundUnnecessary,
    ErrorKind::Unmarshal,
];
fn kind(text: &str) -> Result<ErrorKind, JsValue> {
    KINDS
        .into_iter()
        .find(|k| format!("{k:?}") == text)
        .ok_or_else(|| parameter_error("expected a Rust ErrorKind"))
}
fn text_budget(data: &[u8]) -> Result<(), &'static str> {
    if data.len() > limits::INPUT_BYTES {
        return Err("playground SQL input exceeds 8192 bytes");
    }
    let data = if data.len() > 2 && data.first() == Some(&b'"') && data.last() == Some(&b'"') {
        &data[1..data.len() - 1]
    } else {
        data
    };
    if let Ok(text) = std::str::from_utf8(data) {
        limits::input(text)?;
    }
    Ok(())
}
fn scan<'a>(
    kind: &str,
    text: &'a str,
    number: f64,
    bytes: &'a [u8],
) -> Result<ScanValue<'a>, JsValue> {
    Ok(match kind {
        "Null" => ScanValue::Null,
        "Unsupported" => ScanValue::Unsupported,
        "Float32" => ScanValue::Float32(number as f32),
        "Float64" => ScanValue::Float64(number),
        "Int64" => {
            if text.len() > 20 {
                return Err(parameter_error("value must fit i64"));
            }
            ScanValue::Int64(
                text.parse()
                    .map_err(|_| parameter_error("value must fit i64"))?,
            )
        }
        "Text" => {
            text_budget(text.as_bytes()).map_err(resource_error)?;
            ScanValue::Text(text)
        }
        "Bytes" => {
            text_budget(bytes).map_err(resource_error)?;
            ScanValue::Bytes(bytes)
        }
        _ => return Err(parameter_error("expected a Rust ScanValue variant")),
    })
}
#[wasm_bindgen]
impl Decimal {
    pub fn scan(
        &mut self,
        kind: &str,
        text: &str,
        number: f64,
        bytes: &[u8],
    ) -> Result<(), JsValue> {
        tick()?;
        self.inner
            .scan(scan(kind, text, number, bytes)?)
            .map_err(library_error)
    }
    pub fn value(&self) -> Result<String, JsValue> {
        tick()?;
        Ok(self.inner.value())
    }
}
#[wasm_bindgen]
impl NullDecimal {
    pub fn scan(
        &mut self,
        kind: &str,
        text: &str,
        number: f64,
        bytes: &[u8],
    ) -> Result<(), JsValue> {
        tick()?;
        self.inner
            .scan(scan(kind, text, number, bytes)?)
            .map_err(library_error)
    }
    pub fn value(&self) -> Result<Option<String>, JsValue> {
        tick()?;
        Ok(self.inner.value())
    }
}
#[wasm_bindgen]
pub struct ErrorValue {
    inner: CoreError,
}
#[wasm_bindgen]
impl ErrorValue {
    pub fn new(value_kind: &str, message: &str) -> Result<ErrorValue, JsValue> {
        tick()?;
        if message.len() > limits::INPUT_BYTES {
            return Err(resource_error(
                "playground error message exceeds 8192 bytes",
            ));
        }
        Ok(Self {
            inner: CoreError::new(kind(value_kind)?, message),
        })
    }
    pub fn matches(&self, value_kind: &str) -> Result<bool, JsValue> {
        tick()?;
        Ok(self.inner.matches(kind(value_kind)?))
    }
    pub fn kinds(&self) -> Result<String, JsValue> {
        tick()?;
        Ok(serde_json::to_string(
            &self
                .inner
                .kinds()
                .iter()
                .map(|k| format!("{k:?}"))
                .collect::<Vec<_>>(),
        )
        .unwrap())
    }
    pub fn with_kind(self, value_kind: &str) -> Result<ErrorValue, JsValue> {
        tick()?;
        Ok(Self {
            inner: self.inner.with_kind(kind(value_kind)?),
        })
    }
    pub fn message(&self) -> Result<String, JsValue> {
        tick()?;
        Ok(self.inner.to_string())
    }
}
#[wasm_bindgen]
pub fn error_kinds() -> Result<String, JsValue> {
    tick()?;
    Ok(serde_json::to_string(&KINDS.map(|k| format!("{k:?}"))).unwrap())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quoted_sql_text_is_checked_after_core_quote_removal() {
        assert!(text_budget(b"\"1e100000\"").is_err());
        assert!(text_budget(b"\"1e-100000\"").is_err());
        assert!(text_budget(b"\"1.2300\"").is_ok());
        assert!(text_budget(&[255]).is_ok());
        assert!(text_budget(b"\"\"").is_ok());
    }
}
