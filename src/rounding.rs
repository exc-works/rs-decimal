use crate::{
    Decimal, DecimalError, ErrorKind,
    error::error,
    integer::{floor_div, pow10, precision, trunc_div_rem},
};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Signed, Zero};
use std::cmp::Ordering;

/// The seven modes of the fixed Go contract; discriminants match its constants.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum RoundingMode {
    /// Truncates toward zero.
    Down = 0,
    /// Rounds away from zero.
    Up = 1,
    /// Rounds toward positive infinity.
    Ceiling = 2,
    /// Rounds to nearest, resolving ties away from zero.
    HalfUp = 3,
    /// Rounds to nearest, resolving ties toward zero.
    HalfDown = 4,
    /// Rounds to nearest, resolving ties toward an even retained digit.
    HalfEven = 5,
    /// Requires an exact result; discarding nonzero digits returns `RoundUnnecessary`.
    Unnecessary = 6,
}
impl RoundingMode {
    /// Validates Go's integer mode, returning `InvalidArgument` outside 0 through 6.
    pub fn from_raw(value: i32) -> Result<Self, DecimalError> {
        Ok(match value {
            0 => Self::Down,
            1 => Self::Up,
            2 => Self::Ceiling,
            3 => Self::HalfUp,
            4 => Self::HalfDown,
            5 => Self::HalfEven,
            6 => Self::Unnecessary,
            _ => {
                return Err(DecimalError::new(
                    ErrorKind::InvalidArgument,
                    "invalid rounding mode",
                ));
            }
        })
    }
}

pub(crate) fn rounded_div(
    n: &BigInt,
    d: &BigInt,
    mode: RoundingMode,
) -> Result<BigInt, DecimalError> {
    if d.is_zero() {
        return Err(error(ErrorKind::DivideByZero));
    }
    let (mut q, r) = trunc_div_rem(n, d);
    if r.is_zero() {
        return Ok(q);
    }
    let positive = r.is_positive() == d.is_positive();
    let cmp = (r.abs() << 1usize).cmp(&d.abs());
    let away = match mode {
        RoundingMode::Down => false,
        RoundingMode::Up => true,
        RoundingMode::Ceiling => positive,
        RoundingMode::HalfUp => cmp != Ordering::Less,
        RoundingMode::HalfDown => cmp == Ordering::Greater,
        RoundingMode::HalfEven => {
            cmp == Ordering::Greater || (cmp == Ordering::Equal && q.is_odd())
        }
        RoundingMode::Unnecessary => return Err(error(ErrorKind::RoundUnnecessary)),
    };
    if away {
        q += if positive { 1 } else { -1 };
    }
    Ok(q)
}

pub(crate) fn rounded_div_pow10(
    n: &BigInt,
    exponent: u128,
    mode: RoundingMode,
) -> Result<BigInt, DecimalError> {
    if exponent == 0 || n.is_zero() {
        return Ok(n.clone());
    }
    // For b = bits(|n|) >= 1 and exponent >= b:
    // |n| < 2^b <= 10^exponent / 2. The exact nonzero quotient lies
    // strictly between zero and half a unit, so sign/10 rounds identically.
    // Keep the exponent wide: multiplying scales can exceed isize::MAX.
    if exponent >= n.bits() as u128 {
        return rounded_div(&n.signum(), &BigInt::from(10), mode);
    }
    rounded_div(n, &pow10(exponent as usize), mode)
}

impl Decimal {
    /// Changes decimal places, rounding only when digits must be discarded.
    /// The result is initialized, including when the input is uninitialized zero.
    ///
    /// # Errors
    /// Returns an error for negative precision or required rounding with `Unnecessary`.
    pub fn rescale(&self, prec: isize, mode: RoundingMode) -> Result<Self, DecimalError> {
        let p = precision(prec)?;
        let c = if p == self.scale || self.is_zero() {
            self.raw().clone()
        } else if p > self.scale {
            self.raw() * pow10(p - self.scale)
        } else {
            rounded_div_pow10(self.raw(), (self.scale - p) as u128, mode)?
        };
        Ok(Self::from_parts(c, p))
    }
    /// Changes decimal places, truncating toward zero.
    ///
    /// # Errors
    /// Returns `InvalidPrecision` for negative precision.
    pub fn rescale_down(&self, prec: isize) -> Result<Self, DecimalError> {
        self.rescale(prec, RoundingMode::Down)
    }
    /// Truncates toward zero to scale zero.
    pub fn truncate(&self) -> Self {
        self.truncate_with_prec(0)
            .expect("zero precision and fixed rounding mode are valid")
    }
    /// Truncates toward zero to `prec` decimal places.
    ///
    /// # Errors
    /// Returns `InvalidPrecision` for negative precision.
    pub fn truncate_with_prec(&self, prec: isize) -> Result<Self, DecimalError> {
        self.rescale(prec, RoundingMode::Down)
    }
    /// Rounds half-even to scale zero.
    pub fn round(&self) -> Self {
        self.round_with_prec(0)
            .expect("zero precision and fixed rounding mode are valid")
    }
    /// Rounds half-even to `prec` decimal places.
    ///
    /// # Errors
    /// Returns `InvalidPrecision` for negative precision.
    pub fn round_with_prec(&self, prec: isize) -> Result<Self, DecimalError> {
        self.rescale(prec, RoundingMode::HalfEven)
    }
    /// Rounds toward negative infinity to scale zero.
    pub fn floor(&self) -> Self {
        self.floor_with_prec(0)
            .expect("zero precision and fixed rounding mode are valid")
    }
    /// Rounds toward negative infinity to `prec` decimal places.
    ///
    /// # Errors
    /// Returns `InvalidPrecision` for negative precision.
    pub fn floor_with_prec(&self, prec: isize) -> Result<Self, DecimalError> {
        let p = precision(prec)?;
        if p >= self.scale || self.is_zero() {
            return self.rescale_down(prec);
        }
        if (self.scale - p) as u128 >= self.raw().bits() as u128 {
            return Ok(Self::from_parts(
                BigInt::from(-i32::from(self.is_negative())),
                p,
            ));
        }
        Ok(Self::from_parts(
            floor_div(self.raw(), &pow10(self.scale - p)),
            p,
        ))
    }
    /// Rounds toward positive infinity to scale zero.
    pub fn ceil(&self) -> Self {
        self.ceil_with_prec(0)
            .expect("zero precision and fixed rounding mode are valid")
    }
    /// Rounds toward positive infinity to `prec` decimal places.
    ///
    /// # Errors
    /// Returns `InvalidPrecision` for negative precision.
    pub fn ceil_with_prec(&self, prec: isize) -> Result<Self, DecimalError> {
        self.rescale(prec, RoundingMode::Ceiling)
    }
    /// Multiplies the value by `10^places`, adjusting scale where possible.
    ///
    /// # Errors
    /// Returns `Overflow` if a nonzero shift produces a scale above `i32::MAX`.
    pub fn shift(&self, places: isize) -> Result<Self, DecimalError> {
        if places == 0 {
            return Ok(Self::from_parts(self.raw().clone(), self.scale));
        }
        let target = self.scale as i128 - places as i128;
        if target < 0 {
            return Ok(Self::from_parts(
                if self.is_zero() {
                    BigInt::zero()
                } else {
                    self.raw() * pow10((-target) as usize)
                },
                0,
            ));
        }
        if target > i32::MAX as i128 {
            return Err(error(ErrorKind::Overflow));
        }
        Ok(Self::from_parts(self.raw().clone(), target as usize))
    }
    /// Removes coefficient factors of ten while scale is positive; zero becomes initialized scale-zero zero.
    pub fn strip_trailing_zeros(&self) -> Self {
        if self.is_zero() {
            return Self::new(0);
        }
        if self.scale == 0 || !(self.raw() % 10u8).is_zero() {
            return self.clone();
        }
        let (mut lo, mut hi) = (1, self.scale.min(((self.raw().bits() - 1) / 3) as usize));
        while lo < hi {
            let mid = lo + (hi - lo).div_ceil(2);
            if (self.raw() % pow10(mid)).is_zero() {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        Self::from_parts(self.raw() / pow10(lo), self.scale - lo)
    }
    /// Rounds to at most `figures` significant decimal digits without adding precision.
    ///
    /// # Errors
    /// Returns `InvalidArgument` for nonpositive `figures` or `RoundUnnecessary`
    /// when exact rounding would discard nonzero digits.
    pub fn significant_figures(
        &self,
        figures: isize,
        mode: RoundingMode,
    ) -> Result<Self, DecimalError> {
        if figures <= 0 {
            return Err(error(ErrorKind::InvalidArgument));
        }
        if self.is_zero() {
            return Ok(Self::from_parts(self.raw().clone(), self.scale));
        }
        let digits = self.raw().magnitude().to_string().len();
        let target = figures as i128 - digits as i128 + self.scale as i128;
        if target >= self.scale as i128 {
            return Ok(self.clone());
        }
        if target >= 0 {
            return self.rescale(target as isize, mode);
        }
        let discarded = (self.scale as i128 - target) as usize;
        let c = rounded_div(self.raw(), &pow10(discarded), mode)? * pow10((-target) as usize);
        Ok(Self::from_parts(c, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn power_of_ten_shortcut_matches_exact_division_at_rounding_boundaries() {
        for coefficient in -257..=257 {
            let n = BigInt::from(coefficient);
            for exponent in 0..=10 {
                let denominator = pow10(exponent);
                for raw_mode in 0..=6 {
                    let mode = RoundingMode::from_raw(raw_mode).unwrap();
                    assert_eq!(
                        rounded_div_pow10(&n, exponent as u128, mode),
                        rounded_div(&n, &denominator, mode),
                        "coefficient={coefficient}, exponent={exponent}, mode={mode:?}",
                    );
                }
            }
        }
    }

    #[test]
    fn directed_integer_division_all_signs() {
        for a in -7..=7 {
            for b in [-3, 3] {
                let (a, b) = (BigInt::from(a), BigInt::from(b));
                let (q, r) = trunc_div_rem(&a, &b);
                assert_eq!(&q * &b + &r, a);
                assert!(r.is_zero() || r.sign() == a.sign());
                let floor = floor_div(&a, &b);
                let ceil = crate::integer::ceil_div(&a, &b);
                let rat = num_rational::BigRational::new(a, b);
                assert!(num_rational::BigRational::from_integer(floor) <= rat);
                assert!(num_rational::BigRational::from_integer(ceil) >= rat);
            }
        }
    }
}
