use crate::{Decimal, DecimalError, ErrorKind, NullDecimal};
/// Driver-neutral equivalents of the input types accepted by Go's sql.Scanner.
#[derive(Debug, Clone, Copy)]
pub enum ScanValue<'a> {
    Null,
    Float32(f32),
    Float64(f64),
    Int64(i64),
    Text(&'a str),
    Bytes(&'a [u8]),
    /// Allows dynamic adapters to report an unsupported source type without coercion.
    Unsupported,
}
fn invalid() -> DecimalError {
    DecimalError::new(ErrorKind::Unmarshal, "unsupported or nonfinite SQL value")
}
fn text(data: &[u8]) -> Result<Decimal, DecimalError> {
    let data = if data.len() > 2 && data.first() == Some(&b'"') && data.last() == Some(&b'"') {
        &data[1..data.len() - 1]
    } else {
        data
    };
    Decimal::new_from_string(&String::from_utf8_lossy(data))
        .map_err(|e| e.with_kind(ErrorKind::Unmarshal))
}
impl Decimal {
    pub fn scan(&mut self, value: ScanValue<'_>) -> Result<(), DecimalError> {
        let parsed = match value {
            ScanValue::Null => Self::default(),
            ScanValue::Int64(n) => Self::new(n),
            ScanValue::Float32(f) if f.is_finite() => Self::new_from_float32(f),
            ScanValue::Float64(f) if f.is_finite() => Self::new_from_float64(f),
            ScanValue::Text(s) => text(s.as_bytes())?,
            ScanValue::Bytes(s) => text(s)?,
            _ => return Err(invalid()),
        };
        *self = parsed;
        Ok(())
    }
    /// Canonical SQL text; even an uninitialized Decimal is a non-NULL zero.
    pub fn value(&self) -> String {
        self.string()
    }
}
impl NullDecimal {
    pub fn scan(&mut self, value: ScanValue<'_>) -> Result<(), DecimalError> {
        if matches!(value, ScanValue::Null) {
            *self = Self::default();
            return Ok(());
        }
        self.decimal.scan(value)?;
        self.valid = true;
        Ok(())
    }
    pub fn value(&self) -> Option<String> {
        self.valid.then(|| self.decimal.value())
    }
}
