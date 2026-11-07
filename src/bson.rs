use crate::{BigInt, Decimal, DecimalError, ErrorKind, NullDecimal};
fn invalid() -> DecimalError {
    DecimalError::new(ErrorKind::Unmarshal, "invalid or unsupported BSON value")
}
fn number(text: &str) -> Result<Decimal, DecimalError> {
    Decimal::new_from_string(text).map_err(|e| e.with_kind(ErrorKind::Unmarshal))
}
fn bytes<const N: usize>(data: &[u8]) -> Result<[u8; N], DecimalError> {
    data.get(..N)
        .ok_or_else(invalid)?
        .try_into()
        .map_err(|_| invalid())
}
impl Decimal {
    /// BSON type byte and raw value payload; initialized values encode as BSON strings.
    pub fn marshal_bson_value(&self) -> Result<(u8, Vec<u8>), DecimalError> {
        if self.is_nil() {
            return Ok((10, Vec::new()));
        }
        let text = self.string();
        let len = text
            .len()
            .checked_add(1)
            .and_then(|n| i32::try_from(n).ok())
            .ok_or_else(invalid)?;
        let mut data = len.to_le_bytes().to_vec();
        data.extend_from_slice(text.as_bytes());
        data.push(0);
        Ok((2, data))
    }
    /// Decodes BSON string, double, int32, int64, Decimal128 or null payloads.
    /// Null resets to uninitialized zero. Invalid/unsupported inputs leave the receiver unchanged.
    pub fn unmarshal_bson_value(&mut self, kind: u8, data: &[u8]) -> Result<(), DecimalError> {
        let parsed = match kind {
            10 => Self::default(),
            2 => {
                let len = i32::from_le_bytes(bytes(data)?);
                if len < 1 {
                    return Err(invalid());
                }
                let value = data
                    .get(4..)
                    .and_then(|b| b.get(..len as usize))
                    .ok_or_else(invalid)?;
                // Match Go RawValue.StringValueOK: the final byte is skipped without validating NUL.
                // Invalid UTF-8 becomes invalid decimal text, preserving both error kinds.
                number(&String::from_utf8_lossy(&value[..value.len() - 1]))?
            }
            1 => {
                let f = f64::from_le_bytes(bytes(data)?);
                if !f.is_finite() {
                    return Err(invalid());
                }
                Self::new_from_float64(f)
            }
            16 => Self::new(i64::from(i32::from_le_bytes(bytes(data)?))),
            18 => Self::new(i64::from_le_bytes(bytes(data)?)),
            19 => {
                let raw: [u8; 16] = bytes(data)?;
                let low = u64::from_le_bytes(raw[..8].try_into().unwrap());
                let high = u64::from_le_bytes(raw[8..].try_into().unwrap());
                if matches!((high >> 58) & 31, 30 | 31) {
                    return Err(DecimalError::new(
                        ErrorKind::InvalidFormat,
                        "nonfinite BSON Decimal128",
                    )
                    .with_kind(ErrorKind::Unmarshal));
                }
                let (coefficient, exponent) = if (high >> 61) & 3 == 3 {
                    (0u128, ((high >> 47) & 0x3fff) as i32 - 6176)
                } else {
                    (
                        ((u128::from(high & ((1 << 49) - 1))) << 64) | u128::from(low),
                        ((high >> 49) & 0x3fff) as i32 - 6176,
                    )
                };
                let sign = if high >> 63 != 0 { "-" } else { "" };
                number(&format!("{sign}{coefficient}e{exponent}"))?
            }
            _ => return Err(invalid()),
        };
        *self = parsed;
        Ok(())
    }
}
impl NullDecimal {
    /// Encodes valid values as BSON strings, otherwise BSON null.
    /// Returns an error if the encoded string length exceeds the BSON range.
    pub fn marshal_bson_value(&self) -> Result<(u8, Vec<u8>), DecimalError> {
        if !self.valid {
            return Ok((10, Vec::new()));
        }
        if self.decimal.is_nil() {
            Decimal::new_from_big_int(&BigInt::from(0)).marshal_bson_value()
        } else {
            self.decimal.marshal_bson_value()
        }
    }
    /// Decodes a BSON scalar atomically; BSON null clears both nullable fields.
    pub fn unmarshal_bson_value(&mut self, kind: u8, data: &[u8]) -> Result<(), DecimalError> {
        if kind == 10 {
            *self = Self::default();
            return Ok(());
        }
        self.decimal.unmarshal_bson_value(kind, data)?;
        self.valid = true;
        Ok(())
    }
}
