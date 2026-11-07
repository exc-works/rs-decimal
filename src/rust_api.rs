use crate::{BigInt, Decimal, DecimalError, ErrorKind, RoundingMode};
use std::ops::{Add, Neg, Sub};

/// Moves an arbitrary-size integer into an initialized scale-zero Decimal.
///
/// ```
/// use rs_decimal::{BigInt, Decimal};
/// let coefficient = BigInt::from(123);
/// let d = Decimal::from(coefficient);
/// assert_eq!(d.to_string(), "123");
/// assert_eq!(d.scale(), 0);
/// ```
impl From<BigInt> for Decimal {
    fn from(value: BigInt) -> Self {
        Self::from_parts(value, 0)
    }
}

macro_rules! from_integer {
    ($($integer:ty),* $(,)?) => { $(
        #[doc = concat!(
            "Converts `", stringify!($integer), "` exactly into an initialized scale-zero Decimal.\n\n",
            "```\nuse rs_decimal::Decimal;\nlet input: ", stringify!($integer), " = 42;\n",
            "let d = Decimal::from(input);\nassert_eq!(d.to_string(), \"42\");\n",
            "assert_eq!(d.scale(), 0);\nassert!(!d.is_nil());\n```"
        )]
        impl From<$integer> for Decimal {
            fn from(value: $integer) -> Self {
                Self::from_parts(BigInt::from(value), 0)
            }
        }
    )* };
}
from_integer!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

/// Adds borrowed values exactly at the larger scale, without consuming inputs.
///
/// ```
/// use rs_decimal::Decimal;
/// let a = Decimal::from(2);
/// let b = Decimal::new_with_prec(25, 2);
/// assert_eq!((&a + &b).string_with_trailing_zeros(), "2.25");
/// assert_eq!(a.to_string(), "2");
/// ```
impl Add<&Decimal> for &Decimal {
    type Output = Decimal;
    fn add(self, other: &Decimal) -> Decimal {
        Decimal::add(self, other)
    }
}

/// Subtracts borrowed values exactly at the larger scale, without consuming inputs.
///
/// ```
/// use rs_decimal::Decimal;
/// let a = Decimal::from(2);
/// let b = Decimal::new_with_prec(25, 2);
/// assert_eq!((&a - &b).string_with_trailing_zeros(), "1.75");
/// assert_eq!(b.to_string(), "0.25");
/// ```
impl Sub<&Decimal> for &Decimal {
    type Output = Decimal;
    fn sub(self, other: &Decimal) -> Decimal {
        Decimal::sub(self, other)
    }
}

/// Negates a borrowed value, preserving scale and initializing the result.
///
/// ```
/// use rs_decimal::Decimal;
/// let d = Decimal::new_with_prec(1230, 2);
/// assert_eq!((-&d).string_with_trailing_zeros(), "-12.30");
/// assert_eq!(d.to_string(), "12.3");
/// ```
impl Neg for &Decimal {
    type Output = Decimal;
    fn neg(self) -> Decimal {
        Decimal::neg(self)
    }
}

/// Validates an external mode discriminant without panicking.
/// Values 0 through 6 follow the pinned Go mode mapping; other values return
/// [`ErrorKind::InvalidArgument`]. The legacy [`Self::from_raw`] still panics.
///
/// ```
/// use rs_decimal::{ErrorKind, RoundingMode};
/// assert_eq!(RoundingMode::try_from(5)?, RoundingMode::HalfEven);
/// assert!(RoundingMode::try_from(7).unwrap_err().matches(ErrorKind::InvalidArgument));
/// # Ok::<(), rs_decimal::DecimalError>(())
/// ```
impl TryFrom<i32> for RoundingMode {
    type Error = DecimalError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if (0..=6).contains(&value) {
            Ok(Self::from_raw(value))
        } else {
            Err(DecimalError::new(
                ErrorKind::InvalidArgument,
                "invalid rounding mode",
            ))
        }
    }
}
