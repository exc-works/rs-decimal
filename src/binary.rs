use crate::{Decimal, DecimalError, ErrorKind, integer::MAX_PARSED_PRECISION};
use num_bigint::{BigInt, Sign};
/// Byte length of Go's big-endian precision prefix.
pub const PRECISION_FIXED_SIZE: usize = 4;
impl Decimal {
    /// Go precision prefix followed by math/big.Int Gob version/sign/magnitude.
    pub fn marshal_binary(&self) -> Result<Vec<u8>, DecimalError> {
        if self.is_nil() {
            return Ok(Vec::new());
        }
        let d = self.strip_trailing_zeros();
        if d.scale > MAX_PARSED_PRECISION {
            return Err(DecimalError::new(
                ErrorKind::InvalidPrecision,
                "binary scale exceeds budget",
            ));
        }
        let (sign, mut magnitude) = d.raw().to_bytes_be();
        if d.is_zero() {
            magnitude.clear();
        }
        let mut bytes = (d.scale as u32).to_be_bytes().to_vec();
        bytes.push(if sign == Sign::Minus { 3 } else { 2 });
        bytes.extend(magnitude);
        Ok(bytes)
    }
    pub fn unmarshal_binary(&mut self, bytes: &[u8]) -> Result<(), DecimalError> {
        let invalid =
            || DecimalError::new(ErrorKind::Unmarshal, "invalid Go decimal binary encoding");
        if bytes.is_empty() {
            *self = Self::new(0);
            return Ok(());
        }
        if bytes.len() < PRECISION_FIXED_SIZE {
            return Err(invalid());
        }
        let scale = u32::from_be_bytes(bytes[..4].try_into().unwrap());
        // Fixed-width validation precedes native conversion, including on 32-bit.
        if scale > MAX_PARSED_PRECISION as u32 {
            return Err(invalid());
        }
        let payload = &bytes[4..];
        let c = if payload.is_empty() {
            BigInt::from(0)
        } else {
            if payload[0] >> 1 != 1 {
                return Err(invalid());
            }
            BigInt::from_bytes_be(
                if payload[0] & 1 == 1 {
                    Sign::Minus
                } else {
                    Sign::Plus
                },
                &payload[1..],
            )
        };
        *self = Self::from_parts(c, scale as usize);
        Ok(())
    }
}
