use crate::{ErrorKind, error::fail};
use num_bigint::BigInt;

pub(crate) const MAX_PARSED_PRECISION: usize = 1 << 17;

pub(crate) fn precision(p: isize) -> usize {
    if p < 0 {
        fail(ErrorKind::InvalidPrecision);
    }
    p as usize
}

pub(crate) fn pow10(mut exponent: usize) -> BigInt {
    let mut power = BigInt::from(10);
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
