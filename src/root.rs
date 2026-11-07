use crate::{
    Decimal, DecimalError, ErrorKind, RoundingMode,
    integer::{MAX_PARSED_PRECISION, pow10, precision},
};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::Zero;

fn no_convergence() -> DecimalError {
    DecimalError::new(
        ErrorKind::NoConvergence,
        "root calculation budget exhausted",
    )
}

impl Decimal {
    /// Computes the square root, rounded half-even to `max(self.precision(), 20)` decimal places.
    ///
    /// # Errors
    /// Returns domain errors or `NoConvergence` when rounding cannot be certified within the budget.
    pub fn sqrt(&self) -> Result<Self, DecimalError> {
        self.sqrt_with_prec(self.precision().max(20))
    }
    /// Computes the square root, rounded half-even to `prec` decimal places.
    ///
    /// # Errors
    /// Returns domain errors or `NoConvergence` when rounding cannot be certified within the budget.
    ///
    /// # Panics
    /// Panics with `InvalidPrecision` for negative precision.
    pub fn sqrt_with_prec(&self, prec: isize) -> Result<Self, DecimalError> {
        self.approx_root_with_prec(2, prec)
    }
    /// Computes the positive-degree root (odd degrees allow negative inputs), rounded half-even to `max(self.precision(), 20)` decimal places.
    ///
    /// # Errors
    /// Returns domain errors or `NoConvergence` when rounding cannot be certified within the budget.
    pub fn approx_root(&self, degree: i64) -> Result<Self, DecimalError> {
        self.approx_root_with_prec(degree, self.precision().max(20))
    }
    /// Computes the positive-degree root (odd degrees allow negative inputs), rounded half-even to `prec` decimal places.
    ///
    /// # Errors
    /// Returns domain errors or `NoConvergence` when rounding cannot be certified within the budget.
    ///
    /// # Panics
    /// Panics with `InvalidPrecision` for negative precision.
    pub fn approx_root_with_prec(&self, degree: i64, prec: isize) -> Result<Self, DecimalError> {
        let p = precision(prec);
        if degree <= 0 {
            return Err(DecimalError::new(
                ErrorKind::InvalidRoot,
                "root degree must be positive",
            ));
        }
        if self.is_negative() {
            if degree % 2 == 0 {
                return Err(DecimalError::new(
                    ErrorKind::NegativeRoot,
                    "even root of negative value",
                ));
            }
            return self
                .neg()
                .approx_root_with_prec(degree, prec)
                .map(|d| d.neg());
        }
        if degree == 1 {
            return Ok(self.rescale(prec, RoundingMode::HalfEven));
        }
        if self.is_zero() {
            return Ok(Self::from_parts(BigInt::zero(), p));
        }
        if self == &Self::new(1) {
            return Ok(Self::new_with_append_prec(1, prec));
        }
        if self.scale > MAX_PARSED_PRECISION
            || p > MAX_PARSED_PRECISION
            || degree > MAX_PARSED_PRECISION as i64
            || degree as usize > MAX_PARSED_PRECISION / p.max(1)
        {
            return Err(no_convergence());
        }
        let degree = degree as usize;
        let shift = degree as i128 * p as i128 - self.scale as i128;
        let (mut num, mut den) = (self.raw().clone(), BigInt::from(1));
        if shift >= 0 {
            num *= pow10(shift as usize);
        } else {
            den = pow10((-shift) as usize);
        }
        let mut floor = integer_root(&(&num / &den), degree).ok_or_else(no_convergence)?;
        let right = ((&floor << 1usize) + BigInt::from(1)).pow(degree as u32) * den;
        let cmp = (num << degree).cmp(&right);
        if cmp.is_gt() || (cmp.is_eq() && floor.is_odd()) {
            floor += 1;
        }
        Ok(Self::from_parts(floor, p))
    }
}

fn integer_root(x: &BigInt, n: usize) -> Option<BigInt> {
    if x.is_zero() {
        return Some(BigInt::zero());
    }
    if n == 2 {
        return Some(x.sqrt());
    }
    if n as u128 >= x.bits() as u128 {
        return Some(BigInt::from(1));
    }
    let mut guess = root_upper_bound(x, n);
    for _ in 0..300 {
        let next = (x / guess.pow((n - 1) as u32) + &guess * (n - 1)) / n;
        if next >= guess {
            return Some(guess);
        }
        guess = next;
    }
    None
}

fn root_upper_bound(x: &BigInt, n: usize) -> BigInt {
    let root_bits = ((x.bits() - 1) / n as u64 + 1) as usize;
    let prefix_bits = root_bits.min((usize::BITS - n.leading_zeros()) as usize + 1);
    let shift = root_bits - prefix_bits;
    let prefix = x >> (shift * n);
    let (mut lo, mut hi) = (1u64 << (prefix_bits - 1), 1u64 << prefix_bits);
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        if BigInt::from(mid).pow(n as u32) <= prefix {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    BigInt::from(hi) << shift
}
