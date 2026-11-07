use crate::{
    BigRational, Decimal, DecimalError,
    integer::{MAX_PARSED_PRECISION, ceil_div, floor_div, precision},
    interval::Interval,
    transcendental::{certify, no_convergence},
};
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive};
impl Decimal {
    /// Computes the exponential, rounded half-even to `max(self.precision(), 20)` decimal places.
    ///
    /// # Errors
    /// Returns `NoConvergence` when rounding cannot be certified within the budget.
    pub fn exp(&self) -> Result<Self, DecimalError> {
        self.exp_with_prec(self.precision().max(20))
    }
    /// Computes the exponential, rounded half-even to `prec` decimal places.
    ///
    /// # Errors
    /// Returns `NoConvergence` when rounding cannot be certified within the budget.
    ///
    /// # Panics
    /// Panics with `InvalidPrecision` for negative precision.
    pub fn exp_with_prec(&self, prec: isize) -> Result<Self, DecimalError> {
        let p = precision(prec);
        if p > MAX_PARSED_PRECISION || self.scale > MAX_PARSED_PRECISION {
            return Err(no_convergence());
        }
        if self.is_zero() {
            return Ok(Self::new_with_append_prec(1, prec));
        }
        let negative = self.is_negative();
        let r = self.big_rat().abs();
        if negative && r >= BigRational::new(BigInt::from(5 * (p + 1)), BigInt::from(2)) {
            return Ok(Self::new_with_prec(0, prec));
        }
        let digits = ceil_div(r.numer(), r.denom())
            .to_usize()
            .filter(|n| *n <= MAX_PARSED_PRECISION)
            .ok_or_else(no_convergence)?;
        let mut den = r.denom().clone();
        let twice = r.numer() << 1usize;
        let mut reductions = 0;
        while twice > den {
            den <<= 1usize;
            reductions += 1;
        }
        certify(p, digits, |scale, bits| {
            let x = Interval::fraction(r.numer(), &den, scale);
            let mut value = series(&x, scale, bits)?;
            for _ in 0..reductions {
                value = value.mul_positive(&value, scale);
            }
            if negative {
                value = Interval {
                    lo: scale.clone(),
                    hi: scale.clone(),
                }
                .divide(&value, scale);
            }
            Ok(value)
        })
    }
}
fn series(x: &Interval, scale: &BigInt, max_terms: usize) -> Result<Interval, DecimalError> {
    let mut term = Interval {
        lo: scale.clone(),
        hi: scale.clone(),
    };
    let (mut lo, mut hi) = (scale.clone(), scale.clone());
    for n in 1..=max_terms {
        let product = term.mul_positive(x, scale);
        let divisor = BigInt::from(n);
        term = Interval {
            lo: floor_div(&product.lo, &divisor),
            hi: ceil_div(&product.hi, &divisor),
        };
        lo += &term.lo;
        hi += &term.hi;
        let next = ceil_div(&ceil_div(&(&term.hi * &x.hi), scale), &BigInt::from(n + 1));
        if next <= BigInt::from(2) {
            return Ok(Interval {
                lo,
                hi: hi + (next << 1usize),
            });
        }
    }
    Err(no_convergence())
}
