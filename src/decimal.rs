use crate::{
    DecimalError, ErrorKind,
    error::fail,
    integer::{MAX_PARSED_PRECISION, pow10, precision},
};
use num_bigint::BigInt;
use num_traits::{Signed, Zero};
use std::{cmp::Ordering, fmt, str::FromStr, sync::LazyLock};

static ZERO_INTEGER: LazyLock<BigInt> = LazyLock::new(BigInt::zero);

/// Arbitrary-size coefficient times 10 to the negative scale.
/// Default is an uninitialized numerical zero; constructors initialize it.
#[derive(Clone, Debug, Default)]
pub struct Decimal {
    pub(crate) coefficient: Option<BigInt>,
    pub(crate) scale: usize,
}

/// Valid alone determines nullness; invalid values may retain an inner value.
#[derive(Clone, Debug, Default)]
pub struct NullDecimal {
    pub decimal: Decimal,
    pub valid: bool,
}
impl NullDecimal {
    pub fn new(decimal: Decimal) -> Self {
        Self {
            decimal,
            valid: true,
        }
    }
}

impl Decimal {
    pub(crate) fn from_parts(coefficient: BigInt, scale: usize) -> Self {
        if scale > isize::MAX as usize {
            fail(ErrorKind::Overflow);
        }
        Self {
            coefficient: Some(coefficient),
            scale,
        }
    }
    pub(crate) fn raw(&self) -> &BigInt {
        self.coefficient.as_ref().unwrap_or(&ZERO_INTEGER)
    }
    pub fn new(value: i64) -> Self {
        Self::new_with_prec(value, 0)
    }
    pub fn new_from_int(value: isize) -> Self {
        Self::from_parts(BigInt::from(value), 0)
    }
    pub fn new_from_decimal(value: &Self) -> Self {
        value.clone()
    }
    pub fn new_with_prec(value: i64, prec: isize) -> Self {
        Self::from_parts(BigInt::from(value), precision(prec))
    }
    pub fn new_from_int64(value: i64, prec: isize) -> Self {
        Self::new_with_prec(value, prec)
    }
    pub fn new_from_uint64(value: u64, prec: isize) -> Self {
        Self::from_parts(BigInt::from(value), precision(prec))
    }
    pub fn new_from_big_int(value: &BigInt) -> Self {
        Self::from_parts(value.clone(), 0)
    }
    pub fn new_from_big_int_with_prec(value: &BigInt, prec: isize) -> Self {
        Self::from_parts(value.clone(), precision(prec))
    }
    pub fn new_with_append_prec(value: i64, prec: isize) -> Self {
        let p = precision(prec);
        Self::from_parts(BigInt::from(value) * pow10(p), p)
    }
    pub fn new_from_uint_with_append_prec(value: u64, prec: isize) -> Self {
        let p = precision(prec);
        Self::from_parts(BigInt::from(value) * pow10(p), p)
    }
    pub fn new_from_string(text: &str) -> Result<Self, DecimalError> {
        text.parse()
    }
    pub fn must_from_string(text: &str) -> Self {
        text.parse()
            .unwrap_or_else(|e: DecimalError| std::panic::panic_any(e))
    }
    /// Returns an independent copy of the unscaled integer coefficient.
    pub fn big_int(&self) -> BigInt {
        self.raw().clone()
    }
    /// Decimal places, not significant digits.
    pub fn precision(&self) -> isize {
        self.scale as isize
    }
    pub fn is_nil(&self) -> bool {
        self.coefficient.is_none()
    }
    pub fn sign(&self) -> i32 {
        if self.raw().is_zero() {
            0
        } else if self.raw().is_negative() {
            -1
        } else {
            1
        }
    }
    pub fn is_zero(&self) -> bool {
        self.sign() == 0
    }
    pub fn is_not_zero(&self) -> bool {
        !self.is_zero()
    }
    pub fn is_positive(&self) -> bool {
        self.sign() > 0
    }
    pub fn is_negative(&self) -> bool {
        self.sign() < 0
    }
    pub fn neg(&self) -> Self {
        Self::from_parts(-self.raw(), self.scale)
    }
    pub fn abs(&self) -> Self {
        Self::from_parts(self.raw().abs(), self.scale)
    }
    /// Go-compatible three-way comparison (-1, 0, 1).
    #[allow(clippy::should_implement_trait)]
    pub fn cmp(&self, other: &Self) -> i32 {
        self.cmp_value(other) as i32
    }
    pub fn equal(&self, other: &Self) -> bool {
        self == other
    }
    pub fn not_equal(&self, other: &Self) -> bool {
        self != other
    }
    pub fn gt(&self, other: &Self) -> bool {
        self > other
    }
    pub fn gte(&self, other: &Self) -> bool {
        self >= other
    }
    pub fn lt(&self, other: &Self) -> bool {
        self < other
    }
    pub fn lte(&self, other: &Self) -> bool {
        self <= other
    }
    /// Preserves the first operand's representation on equality.
    pub fn max(&self, other: &Self) -> Self {
        if self >= other {
            self.clone()
        } else {
            other.clone()
        }
    }
    pub fn min(&self, other: &Self) -> Self {
        if self <= other {
            self.clone()
        } else {
            other.clone()
        }
    }

    fn cmp_value(&self, other: &Self) -> Ordering {
        let sign = self.sign();
        let sign2 = other.sign();
        if sign != sign2 {
            return sign.cmp(&sign2);
        }
        if sign == 0 {
            return Ordering::Equal;
        }
        if self.scale == other.scale {
            return self.raw().cmp(other.raw());
        }
        if self.scale > other.scale {
            return other.cmp_value(self).reverse();
        }
        let shift = (other.scale - self.scale) as u128;
        let a = self.raw().bits() as i128;
        let b = other.raw().bits() as i128;
        let gap = b - a;
        if gap < 0 || shift > (gap / 3) as u128 {
            return sign.cmp(&0);
        }
        if gap > 0 && shift <= ((gap - 1) / 4) as u128 {
            return 0.cmp(&sign);
        }
        (self.raw() * pow10(other.scale - self.scale)).cmp(other.raw())
    }

    /// Compact decimal text; full format compatibility is added separately.
    pub fn string(&self) -> String {
        if self.is_zero() {
            return "0".into();
        }
        let negative = self.is_negative();
        let mut digits = self.raw().abs().to_string();
        let mut scale = self.scale;
        while scale > 0 && digits.ends_with('0') {
            digits.pop();
            scale -= 1;
        }
        let mut output = String::new();
        if negative {
            output.push('-');
        }
        if scale == 0 {
            output.push_str(&digits);
        } else if scale >= digits.len() {
            output.push_str("0.");
            output.extend(std::iter::repeat_n('0', scale - digits.len()));
            output.push_str(&digits);
        } else {
            let cut = digits.len() - scale;
            output.push_str(&digits[..cut]);
            output.push('.');
            output.push_str(&digits[cut..]);
        }
        output
    }
}

impl FromStr for Decimal {
    type Err = DecimalError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let invalid = || DecimalError::new(ErrorKind::InvalidFormat, "invalid decimal text");
        let text = text.trim();
        let (mantissa, exponent) = if let Some(i) = text.find(['e', 'E']) {
            (
                &text[..i],
                text[i + 1..].parse::<i32>().map_err(|_| invalid())?,
            )
        } else {
            (text, 0)
        };
        let negative = mantissa.starts_with('-');
        let mantissa = mantissa.strip_prefix(['+', '-']).unwrap_or(mantissa);
        let (whole, fraction) = if let Some((a, b)) = mantissa.split_once('.') {
            if a.is_empty() || b.is_empty() {
                return Err(invalid());
            }
            (a, b)
        } else {
            (mantissa, "")
        };
        if whole.is_empty()
            || !whole
                .bytes()
                .chain(fraction.bytes())
                .all(|c| c.is_ascii_digit())
        {
            return Err(invalid());
        }
        let scale = fraction.len() as i128 - i128::from(exponent);
        if scale.abs() > MAX_PARSED_PRECISION as i128 {
            return Err(invalid());
        }
        let mut c =
            BigInt::parse_bytes(format!("{whole}{fraction}").as_bytes(), 10).ok_or_else(invalid)?;
        if scale < 0 {
            c *= pow10((-scale) as usize);
        }
        if negative {
            c = -c;
        }
        let p = if c.is_zero() {
            0
        } else {
            scale.max(0) as usize
        };
        Ok(Self::from_parts(c, p))
    }
}
impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.string())
    }
}
impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.cmp_value(other) == Ordering::Equal
    }
}
impl Eq for Decimal {}
impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}
impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_value(other)
    }
}
pub fn zero() -> Decimal {
    Decimal::new(0)
}
pub fn one() -> Decimal {
    Decimal::new(1)
}
pub fn ten() -> Decimal {
    Decimal::new(10)
}
pub fn hundred() -> Decimal {
    Decimal::new(100)
}
pub fn max(a: &Decimal, b: &Decimal) -> Decimal {
    Decimal::max(a, b)
}
pub fn min(a: &Decimal, b: &Decimal) -> Decimal {
    Decimal::min(a, b)
}
pub fn between(v: &Decimal, lower: &Decimal, upper: &Decimal) -> bool {
    v >= lower && v <= upper
}
