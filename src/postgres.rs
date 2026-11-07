//! Exact SQLx 0.8 PostgreSQL NUMERIC integration. No float or BigDecimal intermediary.
use crate::{BigInt, Decimal, DecimalError, ErrorKind, NullDecimal, integer::pow10};
use num_integer::Integer;
use num_traits::{Signed, Zero};
use sqlx::postgres::{
    PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, types::Oid,
};
use sqlx::{Decode, Encode, Postgres, Type, ValueRef, encode::IsNull, error::BoxDynError};
use std::fmt::Write;

fn invalid() -> DecimalError {
    DecimalError::new(
        ErrorKind::Unmarshal,
        "invalid or nonfinite PostgreSQL NUMERIC",
    )
}
fn check(value: &Decimal) -> Result<(), DecimalError> {
    if value.scale > 16383 {
        return Err(DecimalError::new(
            ErrorKind::InvalidPrecision,
            "PostgreSQL NUMERIC scale exceeds 16383",
        ));
    }
    if value
        .raw()
        .abs()
        .to_str_radix(10)
        .len()
        .saturating_sub(value.scale)
        > 131072
    {
        return Err(DecimalError::new(
            ErrorKind::Overflow,
            "PostgreSQL NUMERIC integer digits exceed 131072",
        ));
    }
    Ok(())
}
fn encode(value: &Decimal) -> Result<Vec<u8>, DecimalError> {
    check(value)?;
    let scale = value.scale as u16;
    let mut decimal = value.raw().abs().to_str_radix(10);
    let padding = (4 - value.scale % 4) % 4;
    decimal.extend(std::iter::repeat_n('0', padding));
    let left_padding = (4 - decimal.len() % 4) % 4;
    let decimal = "0".repeat(left_padding) + &decimal;
    let mut digits: Vec<u16> = decimal
        .as_bytes()
        .chunks_exact(4)
        .map(|chunk| chunk.iter().fold(0, |n, b| n * 10 + u16::from(b - b'0')))
        .collect();
    let mut weight = digits.len() as i32 - value.scale.div_ceil(4) as i32 - 1;
    while digits.last() == Some(&0) {
        digits.pop();
    }
    if digits.is_empty() {
        weight = 0;
    }
    let weight = i16::try_from(weight)
        .map_err(|_| DecimalError::new(ErrorKind::Overflow, "NUMERIC weight overflow"))?;
    let count = u16::try_from(digits.len())
        .map_err(|_| DecimalError::new(ErrorKind::Overflow, "NUMERIC digit count overflow"))?;
    let sign = if value.raw().is_negative() {
        0x4000u16
    } else {
        0
    };
    let mut out = Vec::with_capacity(8 + digits.len() * 2);
    out.extend_from_slice(&count.to_be_bytes());
    out.extend_from_slice(&weight.to_be_bytes());
    out.extend_from_slice(&sign.to_be_bytes());
    out.extend_from_slice(&scale.to_be_bytes());
    for digit in digits {
        out.extend_from_slice(&digit.to_be_bytes());
    }
    Ok(out)
}
fn decode(data: &[u8]) -> Result<Decimal, DecimalError> {
    if data.len() < 8 {
        return Err(invalid());
    }
    let count = u16::from_be_bytes([data[0], data[1]]) as usize;
    let weight = i16::from_be_bytes([data[2], data[3]]) as i32;
    let sign = u16::from_be_bytes([data[4], data[5]]);
    let scale = u16::from_be_bytes([data[6], data[7]]) as usize;
    if data.len() != 8 + count * 2 || !matches!(sign, 0 | 0x4000) || scale > 16383 {
        return Err(invalid());
    }
    let mut digits = String::with_capacity(count * 4);
    for pair in data[8..].chunks_exact(2) {
        let digit = u16::from_be_bytes([pair[0], pair[1]]);
        if digit >= 10000 {
            return Err(invalid());
        }
        write!(&mut digits, "{digit:04}").unwrap();
    }
    let mut coefficient = if count == 0 {
        BigInt::zero()
    } else {
        BigInt::parse_bytes(digits.as_bytes(), 10).ok_or_else(invalid)?
    };
    if !coefficient.is_zero() {
        let power = (weight + 1 - count as i32) * 4 + scale as i32;
        if power >= 0 {
            coefficient *= pow10(power as usize);
        } else {
            let (q, r) = coefficient.div_rem(&pow10((-power) as usize));
            if !r.is_zero() {
                return Err(invalid());
            }
            coefficient = q;
        }
    }
    if sign == 0x4000 {
        coefficient = -coefficient;
    }
    let value = Decimal::from_parts(coefficient, scale);
    check(&value).map_err(|e| e.with_kind(ErrorKind::Unmarshal))?;
    Ok(value)
}
impl Type<Postgres> for Decimal {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_oid(Oid(1700))
    }
    fn compatible(ty: &PgTypeInfo) -> bool {
        ty.oid() == Some(Oid(1700))
    }
}
impl PgHasArrayType for Decimal {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_oid(Oid(1231))
    }
}
impl Encode<'_, Postgres> for Decimal {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let data = encode(self)?;
        buf.extend_from_slice(&data);
        Ok(IsNull::No)
    }
}
impl<'r> Decode<'r, Postgres> for Decimal {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.format() {
            PgValueFormat::Binary => Ok(decode(value.as_bytes()?)?),
            PgValueFormat::Text => {
                let text = value.as_str()?;
                let mut decoded =
                    Self::new_from_string(text).map_err(|e| e.with_kind(ErrorKind::Unmarshal))?;
                if decoded.is_zero() {
                    let scale = text
                        .split_once('.')
                        .map_or(0, |(_, fraction)| fraction.len());
                    decoded = Self::from_parts(BigInt::zero(), scale);
                }
                check(&decoded)?;
                Ok(decoded)
            }
        }
    }
}
impl Type<Postgres> for NullDecimal {
    fn type_info() -> PgTypeInfo {
        <Decimal as Type<Postgres>>::type_info()
    }
    fn compatible(ty: &PgTypeInfo) -> bool {
        <Decimal as Type<Postgres>>::compatible(ty)
    }
}
impl PgHasArrayType for NullDecimal {
    fn array_type_info() -> PgTypeInfo {
        <Decimal as PgHasArrayType>::array_type_info()
    }
}
impl Encode<'_, Postgres> for NullDecimal {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        if !self.valid {
            Ok(IsNull::Yes)
        } else {
            self.decimal.encode_by_ref(buf)
        }
    }
}
impl<'r> Decode<'r, Postgres> for NullDecimal {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        if value.is_null() {
            Ok(Self::default())
        } else {
            Ok(Self::new(<Decimal as Decode<Postgres>>::decode(value)?))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn numeric_wire_and_atomic_buffer_errors() {
        let d = Decimal::must_from_string("-12345.6700");
        let raw = encode(&d).unwrap();
        assert_eq!(raw, [0, 3, 0, 1, 0x40, 0, 0, 4, 0, 1, 9, 41, 26, 44]);
        assert_eq!(
            decode(&raw).unwrap().string_with_trailing_zeros(),
            "-12345.6700"
        );
        for bytes in [
            &raw[..7],
            &[0, 0, 0, 0, 0xc0, 0, 0, 0],
            &[0, 1, 0, 0, 0, 0, 0, 0, 0xff, 0xff],
        ] {
            assert!(decode(bytes).is_err());
        }
        let mut buf = PgArgumentBuffer::default();
        buf.extend_from_slice(b"prefix");
        assert!(
            Decimal::new_with_prec(1, 16384)
                .encode_by_ref(&mut buf)
                .is_err()
        );
        assert_eq!(&buf[..], b"prefix");
        let zero = Decimal::new_with_prec(0, 4);
        assert_eq!(decode(&encode(&zero).unwrap()).unwrap().precision(), 4);
    }
}
