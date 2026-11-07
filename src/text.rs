use crate::{Decimal, DecimalError, ErrorKind, NullDecimal};
impl Decimal {
    /// Returns canonical decimal ASCII without trailing fractional zeros.
    pub fn marshal_text(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.append_text(&mut out);
        out
    }
    /// Replaces the receiver only after parsing succeeds.
    pub fn unmarshal_text(&mut self, text: &[u8]) -> Result<(), DecimalError> {
        let text = std::str::from_utf8(text).map_err(|_| {
            DecimalError::new(ErrorKind::InvalidFormat, "invalid UTF-8 decimal text")
                .with_kind(ErrorKind::Unmarshal)
        })?;
        let parsed = Self::new_from_string(text).map_err(|e| e.with_kind(ErrorKind::Unmarshal))?;
        *self = parsed;
        Ok(())
    }
    /// Parses a parameter as decimal text; failure leaves the receiver unchanged.
    pub fn unmarshal_param(&mut self, param: &str) -> Result<(), DecimalError> {
        self.unmarshal_text(param.as_bytes())
    }
}
impl NullDecimal {
    /// Returns canonical decimal text if valid, otherwise `null`.
    pub fn string(&self) -> String {
        if self.valid {
            self.decimal.string()
        } else {
            "null".into()
        }
    }
    /// Returns canonical decimal ASCII if valid, otherwise empty bytes.
    pub fn marshal_text(&self) -> Vec<u8> {
        if self.valid {
            self.decimal.marshal_text()
        } else {
            Vec::new()
        }
    }
    /// Decodes text atomically; empty bytes clear validity and the inner value.
    pub fn unmarshal_text(&mut self, text: &[u8]) -> Result<(), DecimalError> {
        if text.is_empty() {
            *self = Self::default();
            return Ok(());
        }
        self.decimal.unmarshal_text(text)?;
        self.valid = true;
        Ok(())
    }
    /// Decodes a parameter atomically; the empty string clears the nullable value.
    pub fn unmarshal_param(&mut self, param: &str) -> Result<(), DecimalError> {
        self.unmarshal_text(param.as_bytes())
    }
}
impl std::fmt::Display for NullDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.string())
    }
}
