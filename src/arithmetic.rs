use crate::{
    Decimal, ErrorKind, RoundingMode as M,
    error::fail,
    integer::{pow10, precision, trunc_div_rem},
    rounding::rounded_div,
};
use num_bigint::BigInt;
use num_traits::{Signed, Zero};

impl Decimal {
    fn aligned(&self, other: &Self) -> (BigInt, BigInt, usize) {
        let p = self.scale.max(other.scale);
        (
            self.rescale_down(p as isize).big_int(),
            other.rescale_down(p as isize).big_int(),
            p,
        )
    }
    pub fn add(&self, other: &Self) -> Self {
        let (a, b, p) = self.aligned(other);
        Self::from_parts(a + b, p)
    }
    pub fn sub(&self, other: &Self) -> Self {
        let (a, b, p) = self.aligned(other);
        Self::from_parts(a - b, p)
    }
    pub fn add_raw(&self, value: i64) -> Self {
        Self::from_parts(self.raw() + value, self.scale)
    }
    pub fn sub_raw(&self, value: i64) -> Self {
        Self::from_parts(self.raw() - value, self.scale)
    }
    pub fn must_non_negative(&self) -> Self {
        assert!(!self.is_negative(), "Negative value");
        Self::from_parts(self.raw().clone(), self.scale)
    }
    pub fn safe_add(&self, other: &Self) -> Self {
        self.add(other).must_non_negative()
    }
    pub fn safe_sub(&self, other: &Self) -> Self {
        self.sub(other).must_non_negative()
    }
    pub fn mul(&self, other: &Self, mode: M) -> Self {
        let product = self.raw() * other.raw();
        let discard = self.scale.min(other.scale);
        let c = if discard > 0 && !product.is_zero() {
            rounded_div(&product, &pow10(discard), mode)
        } else {
            product
        };
        Self::from_parts(c, self.scale.max(other.scale))
    }
    pub fn mul_down(&self, other: &Self) -> Self {
        self.mul(other, M::Down)
    }
    pub fn mul_exact(&self, other: &Self) -> Self {
        let scale = self.scale as u128 + other.scale as u128;
        if scale > i32::MAX as u128 {
            fail(ErrorKind::Overflow);
        }
        Self::from_parts(self.raw() * other.raw(), scale as usize)
    }
    pub fn mul_with_prec(&self, other: &Self, prec: isize, mode: M) -> Self {
        let p = precision(prec);
        let product = self.raw() * other.raw();
        if product.is_zero() {
            return Self::from_parts(product, p);
        }
        let scale = self.scale as u128 + other.scale as u128;
        let c = if scale < p as u128 {
            product * pow10(p - scale as usize)
        } else if scale > p as u128 {
            let discard = scale - p as u128;
            if discard > product.bits() as u128 {
                rounded_div(&product.signum(), &BigInt::from(10), mode)
            } else {
                rounded_div(&product, &pow10(discard as usize), mode)
            }
        } else {
            product
        };
        Self::from_parts(c, p)
    }
    pub fn quo_with_prec(&self, other: &Self, prec: isize, mode: M) -> Self {
        let p = precision(prec);
        if other.is_zero() {
            fail(ErrorKind::DivideByZero);
        }
        if self.is_zero() {
            return Self::from_parts(BigInt::zero(), p);
        }
        let shift = p as i128 - self.scale as i128 + other.scale as i128;
        if shift > isize::MAX as i128 {
            fail(ErrorKind::Overflow);
        }
        let (mut n, mut d) = (self.raw().clone(), other.raw().clone());
        if shift > 0 {
            n *= pow10(shift as usize);
        } else if shift < 0 {
            d *= pow10((-shift) as usize);
        }
        Self::from_parts(rounded_div(&n, &d, mode), p)
    }
    pub fn quo(&self, other: &Self, mode: M) -> Self {
        self.quo_with_prec(other, self.scale.max(other.scale) as isize, mode)
    }
    pub fn quo_down(&self, other: &Self) -> Self {
        self.quo(other, M::Down)
    }
    pub fn quo_rem(&self, other: &Self) -> (Self, Self) {
        // A zero divisor's representation must never force an enormous
        // alignment allocation before the promised DivideByZero panic.
        if other.is_zero() {
            fail(ErrorKind::DivideByZero);
        }
        let (a, b, p) = self.aligned(other);
        let (q, r) = trunc_div_rem(&a, &b);
        (Self::from_parts(q, 0), Self::from_parts(r, p))
    }
    pub fn modulo(&self, other: &Self) -> Self {
        self.quo_rem(other).1
    }
    /// Rust's `mod` keyword requires a raw identifier for the Go-compatible name.
    pub fn r#mod(&self, other: &Self) -> Self {
        self.modulo(other)
    }
    pub fn int_part(&self) -> BigInt {
        if self.scale == 0 {
            return self.raw().clone();
        }
        if self.scale as u128 >= self.raw().bits() as u128 {
            return BigInt::zero();
        }
        self.raw() / pow10(self.scale)
    }
    /// The fractional component is an unscaled signed integer remainder.
    pub fn remainder(&self) -> (BigInt, BigInt) {
        if self.scale as u128 >= self.raw().bits() as u128 {
            return (BigInt::zero(), self.raw().clone());
        }
        trunc_div_rem(self.raw(), &pow10(self.scale))
    }
    pub fn is_integer(&self) -> bool {
        if self.scale == 0 || self.is_zero() {
            return true;
        }
        if self.scale as u128 >= self.raw().bits() as u128 {
            return false;
        }
        (self.raw() % pow10(self.scale)).is_zero()
    }
    pub fn has_fraction(&self) -> bool {
        !self.is_integer()
    }
    pub fn power(&self, power: i64) -> Self {
        if power == 0 {
            return Self::new(1).rescale(self.precision(), M::Unnecessary);
        }
        if self.is_zero() {
            if power < 0 {
                fail(ErrorKind::DivideByZero);
            }
            return Self::from_parts(BigInt::zero(), self.scale);
        }
        let base = self.strip_trailing_zeros();
        let magnitude = power.unsigned_abs();
        if base.scale == 0 && base.raw().abs() == BigInt::from(1) {
            let sign = if base.is_negative() && magnitude % 2 != 0 {
                -1
            } else {
                1
            };
            return Self::new(sign).rescale(self.precision(), M::Unnecessary);
        }
        let size = (base.scale as u128).max(base.raw().bits() as u128).max(1);
        if magnitude as u128 > i32::MAX as u128 / size {
            fail(ErrorKind::Overflow);
        }
        let exact = Self::from_parts(
            base.raw().pow(magnitude as u32),
            base.scale * magnitude as usize,
        );
        if power < 0 {
            Self::new(1).quo_with_prec(&exact, self.precision(), M::HalfEven)
        } else {
            exact.rescale(self.precision(), M::HalfEven)
        }
    }
}
