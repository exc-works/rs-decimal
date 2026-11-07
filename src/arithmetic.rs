use crate::{
    Decimal, ErrorKind, RoundingMode,
    error::fail,
    integer::{pow10, precision, trunc_div_rem},
    rounding::rounded_div,
};
use num_bigint::BigInt;
use num_traits::{Signed, Zero};
use std::borrow::Cow;

impl Decimal {
    // Alignment only increases scale, so it cannot discard digits. Borrow the
    // original coefficient when possible; allocate only for a nonzero shift.
    fn coefficient_at_scale(&self, scale: usize) -> Cow<'_, BigInt> {
        debug_assert!(scale >= self.scale);
        if scale == self.scale || self.is_zero() {
            Cow::Borrowed(self.raw())
        } else {
            Cow::Owned(self.raw() * pow10(scale - self.scale))
        }
    }

    /// Adds exactly at the larger operand scale.
    pub fn add(&self, other: &Self) -> Self {
        let scale = self.scale.max(other.scale);
        let left = self.coefficient_at_scale(scale);
        let right = other.coefficient_at_scale(scale);
        let coefficient = match (left, right) {
            (Cow::Owned(left), right) => left + right.as_ref(),
            (Cow::Borrowed(left), Cow::Owned(right)) => left + right,
            (Cow::Borrowed(left), Cow::Borrowed(right)) => left + right,
        };
        Self::from_parts(coefficient, scale)
    }
    /// Subtracts exactly at the larger operand scale.
    pub fn sub(&self, other: &Self) -> Self {
        let scale = self.scale.max(other.scale);
        let left = self.coefficient_at_scale(scale);
        let right = other.coefficient_at_scale(scale);
        let coefficient = match (left, right) {
            (Cow::Owned(left), right) => left - right.as_ref(),
            (Cow::Borrowed(left), Cow::Owned(right)) => left - right,
            (Cow::Borrowed(left), Cow::Borrowed(right)) => left - right,
        };
        Self::from_parts(coefficient, scale)
    }
    /// Adds to the unscaled coefficient, preserving scale. At scale 2, adding 1 adds 0.01.
    pub fn add_raw(&self, value: i64) -> Self {
        Self::from_parts(self.raw() + value, self.scale)
    }
    /// Subtracts from the unscaled coefficient, preserving scale.
    pub fn sub_raw(&self, value: i64) -> Self {
        Self::from_parts(self.raw() - value, self.scale)
    }
    /// Returns an initialized copy of a nonnegative value.
    ///
    /// # Panics
    /// Panics with a string payload when the value is negative.
    pub fn must_non_negative(&self) -> Self {
        assert!(!self.is_negative(), "Negative value");
        Self::from_parts(self.raw().clone(), self.scale)
    }
    /// Adds exactly, then requires a nonnegative result.
    ///
    /// # Panics
    /// Panics with a string payload when the sum is negative.
    pub fn safe_add(&self, other: &Self) -> Self {
        let result = self.add(other);
        assert!(!result.is_negative(), "Negative value");
        result
    }
    /// Subtracts exactly, then requires a nonnegative result.
    ///
    /// # Panics
    /// Panics with a string payload when the difference is negative.
    pub fn safe_sub(&self, other: &Self) -> Self {
        let result = self.sub(other);
        assert!(!result.is_negative(), "Negative value");
        result
    }
    /// Multiplies and rounds to the larger operand scale.
    /// Use [`Self::mul_exact`] to retain every product digit.
    ///
    /// # Panics
    /// Panics with `RoundUnnecessary` if the chosen mode forbids required rounding.
    pub fn mul(&self, other: &Self, mode: RoundingMode) -> Self {
        let product = self.raw() * other.raw();
        let discard = self.scale.min(other.scale);
        let c = if discard > 0 && !product.is_zero() {
            rounded_div(&product, &pow10(discard), mode)
        } else {
            product
        };
        Self::from_parts(c, self.scale.max(other.scale))
    }
    /// Multiplies at the larger operand scale, truncating toward zero.
    pub fn mul_down(&self, other: &Self) -> Self {
        self.mul(other, RoundingMode::Down)
    }
    /// Multiplies exactly; the result scale is the sum of the operand scales.
    ///
    /// # Panics
    /// Panics with `Overflow` when the sum of scales exceeds `i32::MAX`.
    pub fn mul_exact(&self, other: &Self) -> Self {
        let scale = self.scale as u128 + other.scale as u128;
        if scale > i32::MAX as u128 {
            fail(ErrorKind::Overflow);
        }
        Self::from_parts(self.raw() * other.raw(), scale as usize)
    }
    /// Multiplies and rounds to `prec` decimal places.
    ///
    /// # Panics
    /// Panics for negative precision or required rounding with `Unnecessary`.
    pub fn mul_with_prec(&self, other: &Self, prec: isize, mode: RoundingMode) -> Self {
        let p = precision(prec);
        let product = self.raw() * other.raw();
        if product.is_zero() {
            return Self::from_parts(product, p);
        }
        let scale = self.scale as u128 + other.scale as u128;
        let c = match scale.cmp(&(p as u128)) {
            std::cmp::Ordering::Less => product * pow10(p - scale as usize),
            std::cmp::Ordering::Greater => {
                let discard = scale - p as u128;
                if discard > product.bits() as u128 {
                    rounded_div(&product.signum(), &BigInt::from(10), mode)
                } else {
                    rounded_div(&product, &pow10(discard as usize), mode)
                }
            }
            std::cmp::Ordering::Equal => product,
        };
        Self::from_parts(c, p)
    }
    /// Divides and rounds to `prec` decimal places.
    ///
    /// # Panics
    /// Panics for a zero divisor, negative precision, a scaling shift above
    /// `isize::MAX`, or required rounding with `Unnecessary`.
    pub fn quo_with_prec(&self, other: &Self, prec: isize, mode: RoundingMode) -> Self {
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
        let coefficient = match shift.cmp(&0) {
            std::cmp::Ordering::Greater => {
                rounded_div(&(self.raw() * pow10(shift as usize)), other.raw(), mode)
            }
            std::cmp::Ordering::Less => {
                rounded_div(self.raw(), &(other.raw() * pow10((-shift) as usize)), mode)
            }
            std::cmp::Ordering::Equal => rounded_div(self.raw(), other.raw(), mode),
        };
        Self::from_parts(coefficient, p)
    }
    /// Divides at the larger operand scale. See [`Self::quo_with_prec`] for panic conditions.
    pub fn quo(&self, other: &Self, mode: RoundingMode) -> Self {
        self.quo_with_prec(other, self.scale.max(other.scale) as isize, mode)
    }
    /// Divides at the larger operand scale, truncating toward zero.
    ///
    /// # Panics
    /// Panics for a zero divisor or an overflowing scaling shift.
    pub fn quo_down(&self, other: &Self) -> Self {
        self.quo(other, RoundingMode::Down)
    }
    /// Returns a quotient truncated toward zero and a remainder at the larger operand scale.
    /// The remainder has the dividend sign and satisfies `self = quotient * other + remainder`.
    ///
    /// # Panics
    /// Panics with `DivideByZero` for a zero divisor.
    pub fn quo_rem(&self, other: &Self) -> (Self, Self) {
        // A zero divisor's representation must never force an enormous
        // alignment allocation before the promised DivideByZero panic.
        if other.is_zero() {
            fail(ErrorKind::DivideByZero);
        }
        let scale = self.scale.max(other.scale);
        let dividend = self.coefficient_at_scale(scale);
        let divisor = other.coefficient_at_scale(scale);
        let (quotient, remainder) = trunc_div_rem(&dividend, &divisor);
        (
            Self::from_parts(quotient, 0),
            Self::from_parts(remainder, scale),
        )
    }
    /// Returns the signed remainder from [`Self::quo_rem`].
    ///
    /// # Panics
    /// Panics with `DivideByZero` for a zero divisor.
    pub fn modulo(&self, other: &Self) -> Self {
        self.quo_rem(other).1
    }
    /// Rust's `mod` keyword requires a raw identifier for the Go-compatible name.
    pub fn r#mod(&self, other: &Self) -> Self {
        self.modulo(other)
    }
    /// Returns the integer part truncated toward zero.
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
    /// Tests whether the numerical value has no fractional component.
    pub fn is_integer(&self) -> bool {
        if self.scale == 0 || self.is_zero() {
            return true;
        }
        if self.scale as u128 >= self.raw().bits() as u128 {
            return false;
        }
        (self.raw() % pow10(self.scale)).is_zero()
    }
    /// Tests whether the numerical value has a nonzero fractional component.
    pub fn has_fraction(&self) -> bool {
        !self.is_integer()
    }
    /// Raises to an integer power and rounds half-even to the original scale.
    /// Zero to power zero is one; negative powers use the reciprocal.
    ///
    /// # Panics
    /// Panics with `DivideByZero` for zero to a negative power or `Overflow`
    /// when the intermediate coefficient/scale budget is exceeded.
    pub fn power(&self, power: i64) -> Self {
        if power == 0 {
            return Self::new(1).rescale(self.precision(), RoundingMode::Unnecessary);
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
            return Self::new(sign).rescale(self.precision(), RoundingMode::Unnecessary);
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
            Self::new(1).quo_with_prec(&exact, self.precision(), RoundingMode::HalfEven)
        } else {
            exact.rescale(self.precision(), RoundingMode::HalfEven)
        }
    }
}
