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
    Down = 0,
    Up = 1,
    Ceiling = 2,
    HalfUp = 3,
    HalfDown = 4,
    HalfEven = 5,
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
    pub fn rescale_down(&self, prec: isize) -> Self {
        self.rescale(prec, RoundingMode::Down)
    }
    pub fn truncate(&self) -> Self {
        self.truncate_with_prec(0)
    }
    pub fn truncate_with_prec(&self, prec: isize) -> Self {
        self.rescale(prec, RoundingMode::Down)
    }
    pub fn round(&self) -> Self {
        self.round_with_prec(0)
    }
    pub fn round_with_prec(&self, prec: isize) -> Self {
        self.rescale(prec, RoundingMode::HalfEven)
    }
    pub fn floor(&self) -> Self {
        self.floor_with_prec(0)
    }
    pub fn floor_with_prec(&self, prec: isize) -> Self {
        let p = precision(prec);
        if p >= self.scale || self.is_zero() {
            return self.rescale_down(prec);
        }
        Self::from_parts(floor_div(self.raw(), &pow10(self.scale - p)), p)
    }
    pub fn ceil(&self) -> Self {
        self.ceil_with_prec(0)
    }
    pub fn ceil_with_prec(&self, prec: isize) -> Self {
        let p = precision(prec);
        if p >= self.scale || self.is_zero() {
            return self.rescale_down(prec);
        }
        Self::from_parts(ceil_div(self.raw(), &pow10(self.scale - p)), p)
    }
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
    pub fn significant_figures(&self, figures: isize, mode: RoundingMode) -> Self {
        assert!(figures > 0, "figures must be greater than 0");
        if self.is_zero() {
            return Self::from_parts(self.raw().clone(), self.scale);
        }
        let digits = self.raw().abs().to_string().len();
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
