use crate::{DecimalError, ErrorKind, error::error};
use num_bigint::BigInt;
use num_integer::Integer;

// Text final-scale magnitude and Go binary scale are wire-format boundaries.
// They are not limits on coefficients or direct constructors.
pub(crate) const MAX_WIRE_SCALE: usize = 1 << 17;

// Root/log/exp algorithms use a separate decimal working-size budget.
// Equal values do not make the wire and calculation policies interchangeable.
pub(crate) const MAX_CALCULATION_DIGITS: usize = 1 << 17;

pub(crate) fn precision(p: isize) -> Result<usize, DecimalError> {
    if p < 0 {
        return Err(error(ErrorKind::InvalidPrecision));
    }
    Ok(p as usize)
}

pub(crate) fn pow10(exponent: usize) -> BigInt {
    pow_integer(BigInt::from(10), exponent)
}

pub(crate) fn pow_integer(mut power: BigInt, mut exponent: usize) -> BigInt {
    let mut value = BigInt::from(1);
    while exponent > 0 {
        if exponent & 1 != 0 {
            value *= &power;
        }
        exponent >>= 1;
        if exponent > 0 {
            power = &power * &power;
        }
    }
    value
}

pub(crate) fn trunc_div_rem(a: &BigInt, b: &BigInt) -> (BigInt, BigInt) {
    a.div_rem(b)
}
pub(crate) fn floor_div(a: &BigInt, b: &BigInt) -> BigInt {
    a.div_floor(b)
}
pub(crate) fn ceil_div(a: &BigInt, b: &BigInt) -> BigInt {
    a.div_ceil(b)
}
