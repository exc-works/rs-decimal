use crate::{
    Decimal, DecimalError, ErrorKind, RoundingMode,
    integer::{pow10, precision},
    rounding::rounded_div,
};
use num_bigint::BigInt;
use num_traits::Zero;
use std::borrow::Cow;

impl Decimal {
    /// Borrows the unscaled coefficient without copying its digits.
    /// Uninitialized zero borrows a shared integer zero; use [`Self::is_nil`]
    /// separately when initialization matters.
    ///
    /// ```
    /// use rs_decimal::{BigInt, Decimal};
    /// let d = Decimal::new_with_prec(123, 2);
    /// assert_eq!(d.coefficient(), &BigInt::from(123));
    /// ```
    ///
    /// A shared coefficient cannot be mutated:
    /// ```compile_fail
    /// use rs_decimal::Decimal;
    /// let mut d = Decimal::new(123);
    /// *d.coefficient() += 1;
    /// ```
    pub fn coefficient(&self) -> &BigInt {
        self.raw()
    }

    /// Moves out the coefficient, discarding scale and initialization state.
    /// Uninitialized zero yields integer zero. Use [`Self::coefficient`] to borrow.
    ///
    /// ```
    /// use rs_decimal::{BigInt, Decimal};
    /// assert_eq!(Decimal::new_with_prec(123, 2).into_coefficient(), BigInt::from(123));
    /// ```
    ///
    /// The consumed Decimal is no longer available:
    /// ```compile_fail
    /// use rs_decimal::Decimal;
    /// let d = Decimal::new(123);
    /// let coefficient = d.into_coefficient();
    /// println!("{d}");
    /// ```
    pub fn into_coefficient(self) -> BigInt {
        self.coefficient.unwrap_or_default()
    }

    /// Takes ownership of a coefficient at a nonnegative decimal-place count.
    /// Always initializes the result, retaining scale even for zero. This does
    /// not apply the text parser's scale cap or remove trailing zeros.
    ///
    /// # Errors
    /// Returns [`ErrorKind::Overflow`] when scale exceeds `isize::MAX`.
    ///
    /// ```
    /// use rs_decimal::{BigInt, Decimal};
    /// let d = Decimal::try_from_coefficient(BigInt::from(12300), 4)?;
    /// assert_eq!(d.string_with_trailing_zeros(), "1.2300");
    /// # Ok::<(), rs_decimal::DecimalError>(())
    /// ```
    pub fn try_from_coefficient(coefficient: BigInt, scale: usize) -> Result<Self, DecimalError> {
        if scale > isize::MAX as usize {
            return Err(DecimalError::new(
                ErrorKind::Overflow,
                "decimal scale exceeds isize::MAX",
            ));
        }
        Ok(Self::from_parts(coefficient, scale))
    }

    /// Returns the nonnegative decimal-place count, not significant digits.
    ///
    /// ```
    /// use rs_decimal::Decimal;
    /// assert_eq!(Decimal::new_with_prec(12300, 4).scale(), 4);
    /// ```
    pub fn scale(&self) -> usize {
        self.scale
    }

    /// Consumes this value and changes decimal places with [`Self::rescale`]'s
    /// rounding and initialization rules. Unchanged-scale results reuse the
    /// owned coefficient; even uninitialized zero becomes initialized.
    ///
    /// # Panics
    /// Panics with `InvalidPrecision` for negative precision or
    /// `RoundUnnecessary` when exact rounding would discard nonzero digits.
    ///
    /// ```
    /// use rs_decimal::{Decimal, RoundingMode};
    /// let d = Decimal::new_with_prec(12345, 3).into_rescaled(2, RoundingMode::HalfEven);
    /// assert_eq!(d.string_with_trailing_zeros(), "12.34");
    /// ```
    pub fn into_rescaled(self, prec: isize, mode: RoundingMode) -> Self {
        let target = precision(prec);
        let scale = self.scale;
        let coefficient = self.into_coefficient();
        let coefficient = if target == scale || coefficient.is_zero() {
            coefficient
        } else if target > scale {
            coefficient * pow10(target - scale)
        } else {
            rounded_div(&coefficient, &pow10(scale - target), mode)
        };
        Self::from_parts(coefficient, target)
    }

    // Alignment only increases scale. Zero must bypass the power even at the
    // native maximum; moving out default zero still initializes the result.
    fn into_coefficient_at_scale(self, scale: usize) -> BigInt {
        debug_assert!(scale >= self.scale);
        let shift = scale - self.scale;
        let coefficient = self.into_coefficient();
        if shift == 0 || coefficient.is_zero() {
            coefficient
        } else {
            coefficient * pow10(shift)
        }
    }

    /// Consumes the left operand and borrows the right for exact addition.
    /// Returns an initialized result at the larger scale, like [`Self::add`].
    /// Owned coefficient storage is reused where possible; growth can allocate.
    ///
    /// ```
    /// use rs_decimal::Decimal;
    /// let rhs = Decimal::new_with_prec(25, 2);
    /// let sum = Decimal::new(2).add_owned(&rhs);
    /// assert_eq!(sum.string_with_trailing_zeros(), "2.25");
    /// assert_eq!(rhs.string(), "0.25");
    /// ```
    pub fn add_owned(self, other: &Self) -> Self {
        let scale = self.scale.max(other.scale);
        let left = self.into_coefficient_at_scale(scale);
        let right = other.coefficient_at_scale(scale);
        let coefficient = match right {
            Cow::Borrowed(right) => left + right,
            Cow::Owned(right) => left + right,
        };
        Self::from_parts(coefficient, scale)
    }

    /// Consumes the left operand and borrows the right for exact subtraction.
    /// Returns an initialized result at the larger scale, like [`Self::sub`].
    /// Owned coefficient storage is reused where possible; growth can allocate.
    ///
    /// ```
    /// use rs_decimal::Decimal;
    /// let rhs = Decimal::new_with_prec(25, 2);
    /// let difference = Decimal::new(2).sub_owned(&rhs);
    /// assert_eq!(difference.string_with_trailing_zeros(), "1.75");
    /// assert_eq!(rhs.string(), "0.25");
    /// ```
    pub fn sub_owned(self, other: &Self) -> Self {
        let scale = self.scale.max(other.scale);
        let left = self.into_coefficient_at_scale(scale);
        let right = other.coefficient_at_scale(scale);
        let coefficient = match right {
            Cow::Borrowed(right) => left - right,
            Cow::Owned(right) => left - right,
        };
        Self::from_parts(coefficient, scale)
    }

    /// Borrows the larger operand, selecting self on numerical equality.
    /// Selection preserves the chosen operand's scale and initialization.
    ///
    /// ```
    /// use rs_decimal::Decimal;
    /// let a = Decimal::new_with_prec(100, 2);
    /// let b = Decimal::new(1);
    /// assert!(std::ptr::eq(a.max_ref(&b), &a));
    /// ```
    pub fn max_ref<'a>(&'a self, other: &'a Self) -> &'a Self {
        if self >= other { self } else { other }
    }

    /// Borrows the smaller operand, selecting self on numerical equality.
    /// Selection preserves the chosen operand's scale and initialization.
    ///
    /// ```
    /// use rs_decimal::Decimal;
    /// let a = Decimal::default();
    /// let b = Decimal::new(0);
    /// assert!(std::ptr::eq(a.min_ref(&b), &a));
    /// ```
    pub fn min_ref<'a>(&'a self, other: &'a Self) -> &'a Self {
        if self <= other { self } else { other }
    }
}
