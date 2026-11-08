use crate::{
    Decimal, ErrorKind,
    error::fail,
    integer::{ceil_div, floor_div, pow10, precision, trunc_div_rem},
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
    /// Requires an exact result; discarding nonzero digits panics.
    Unnecessary = 6,
}
impl RoundingMode {
    /// Compatibility boundary for Go's integer mode. Invalid values panic.
    pub fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Down,
            1 => Self::Up,
            2 => Self::Ceiling,
            3 => Self::HalfUp,
            4 => Self::HalfDown,
            5 => Self::HalfEven,
            6 => Self::Unnecessary,
            _ => panic!("invalid rounding mode"),
        }
    }
}

pub(crate) fn rounded_div(n: &BigInt, d: &BigInt, mode: RoundingMode) -> BigInt {
    if d.is_zero() {
        fail(ErrorKind::DivideByZero);
    }
    let (mut q, r) = trunc_div_rem(n, d);
    if r.is_zero() {
        return q;
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
        RoundingMode::Unnecessary => fail(ErrorKind::RoundUnnecessary),
    };
    if away {
        q += if positive { 1 } else { -1 };
    }
    q
}

impl Decimal {
    /// Changes decimal places, rounding only when digits must be discarded.
    /// The result is initialized, including when the input is uninitialized zero.
    ///
    /// # Panics
    /// Panics for negative precision or required rounding with `Unnecessary`.
    pub fn rescale(&self, prec: isize, mode: RoundingMode) -> Self {
        let p = precision(prec);
        let c = if p == self.scale || self.is_zero() {
            self.raw().clone()
        } else if p > self.scale {
            self.raw() * pow10(p - self.scale)
        } else {
            rounded_div(self.raw(), &pow10(self.scale - p), mode)
        };
        Self::from_parts(c, p)
    }
    /// Changes decimal places, truncating toward zero.
    ///
    /// # Panics
    /// Panics with `InvalidPrecision` for negative precision.
    pub fn rescale_down(&self, prec: isize) -> Self {
        self.rescale(prec, RoundingMode::Down)
    }
    /// Truncates toward zero to scale zero.
    pub fn truncate(&self) -> Self {
        self.truncate_with_prec(0)
    }
    /// Truncates toward zero to `prec` decimal places.
    ///
    /// # Panics
    /// Panics with `InvalidPrecision` for negative precision.
    pub fn truncate_with_prec(&self, prec: isize) -> Self {
        self.rescale(prec, RoundingMode::Down)
    }
    /// Rounds half-even to scale zero.
    pub fn round(&self) -> Self {
        self.round_with_prec(0)
    }
    /// Rounds half-even to `prec` decimal places.
    ///
    /// # Panics
    /// Panics with `InvalidPrecision` for negative precision.
    pub fn round_with_prec(&self, prec: isize) -> Self {
        self.rescale(prec, RoundingMode::HalfEven)
    }
    /// Rounds toward negative infinity to scale zero.
    pub fn floor(&self) -> Self {
        self.floor_with_prec(0)
    }
    /// Rounds toward negative infinity to `prec` decimal places.
    ///
    /// # Panics
    /// Panics with `InvalidPrecision` for negative precision.
    pub fn floor_with_prec(&self, prec: isize) -> Self {
        let p = precision(prec);
        if p >= self.scale || self.is_zero() {
            return self.rescale_down(prec);
        }
        Self::from_parts(floor_div(self.raw(), &pow10(self.scale - p)), p)
    }
    /// Rounds toward positive infinity to scale zero.
    pub fn ceil(&self) -> Self {
        self.ceil_with_prec(0)
    }
    /// Rounds toward positive infinity to `prec` decimal places.
    ///
    /// # Panics
    /// Panics with `InvalidPrecision` for negative precision.
    pub fn ceil_with_prec(&self, prec: isize) -> Self {
        let p = precision(prec);
        if p >= self.scale || self.is_zero() {
            return self.rescale_down(prec);
        }
        Self::from_parts(ceil_div(self.raw(), &pow10(self.scale - p)), p)
    }
    /// Multiplies the value by `10^places`, adjusting scale where possible.
    ///
    /// # Panics
    /// Panics with `Overflow` if a nonzero shift produces a scale above `i32::MAX`.
    pub fn shift(&self, places: isize) -> Self {
        if places == 0 {
            return Self::from_parts(self.raw().clone(), self.scale);
        }
        let target = self.scale as i128 - places as i128;
        if target < 0 {
            return Self::from_parts(
                if self.is_zero() {
                    BigInt::zero()
                } else {
                    self.raw() * pow10((-target) as usize)
                },
                0,
            );
        }
        if target > i32::MAX as i128 {
            fail(ErrorKind::Overflow);
        }
        Self::from_parts(self.raw().clone(), target as usize)
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
    /// # Panics
    /// Panics for nonpositive `figures` or required rounding with `Unnecessary`.
    pub fn significant_figures(&self, figures: isize, mode: RoundingMode) -> Self {
        assert!(figures > 0, "figures must be greater than 0");
        if self.is_zero() {
            return Self::from_parts(self.raw().clone(), self.scale);
        }
        let digits = self.raw().magnitude().to_string().len();
        let target = figures as i128 - digits as i128 + self.scale as i128;
        if target >= self.scale as i128 {
            return self.clone();
        }
        if target >= 0 {
            return self.rescale(target as isize, mode);
        }
        let discarded = (self.scale as i128 - target) as usize;
        let c = rounded_div(self.raw(), &pow10(discarded), mode) * pow10((-target) as usize);
        Self::from_parts(c, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn directed_integer_division_all_signs() {
        for a in -7..=7 {
            for b in [-3, 3] {
                let (a, b) = (BigInt::from(a), BigInt::from(b));
                let (q, r) = trunc_div_rem(&a, &b);
                assert_eq!(&q * &b + &r, a);
                assert!(r.is_zero() || r.sign() == a.sign());
                let floor = floor_div(&a, &b);
                let ceil = ceil_div(&a, &b);
                let rat = num_rational::BigRational::new(a, b);
                assert!(num_rational::BigRational::from_integer(floor) <= rat);
                assert!(num_rational::BigRational::from_integer(ceil) >= rat);
            }
        }
    }
}
