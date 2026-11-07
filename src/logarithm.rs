use crate::{
    Decimal, DecimalError, ErrorKind,
    integer::{MAX_PARSED_PRECISION, ceil_div, floor_div, precision},
    interval::Interval,
    transcendental::{certify, no_convergence},
};
use num_bigint::BigInt;
use num_traits::{One, Zero};

impl Decimal {
    /// Computes the natural logarithm, rounded half-even to `max(self.precision(), 20)` decimal places.
    ///
    /// # Errors
    /// Returns domain errors or `NoConvergence` when rounding cannot be certified within the budget.
    pub fn ln(&self) -> Result<Self, DecimalError> {
        self.ln_with_prec(self.precision().max(20))
    }
    /// Computes the natural logarithm, rounded half-even to `prec` decimal places.
    ///
    /// # Errors
    /// Returns domain errors or `NoConvergence` when rounding cannot be certified within the budget.
    ///
    /// # Panics
    /// Panics with `InvalidPrecision` for negative precision.
    pub fn ln_with_prec(&self, prec: isize) -> Result<Self, DecimalError> {
        self.certified_log(prec, 0)
    }
    /// Computes the base-two logarithm at `max(self.precision(), 20)` decimal places.
    ///
    /// # Panics
    /// Panics with the [`DecimalError`] returned by [`Self::log2_with_prec`].
    /// Use that method when the caller needs to handle domain or convergence errors.
    pub fn log2(&self) -> Self {
        self.log2_with_prec(self.precision().max(20))
            .unwrap_or_else(|e| std::panic::panic_any(e))
    }
    /// Computes the base-two logarithm, rounded half-even to `prec` decimal places.
    ///
    /// # Errors
    /// Returns domain errors or `NoConvergence` when rounding cannot be certified within the budget.
    ///
    /// # Panics
    /// Panics with `InvalidPrecision` for negative precision.
    pub fn log2_with_prec(&self, prec: isize) -> Result<Self, DecimalError> {
        self.certified_log(prec, 2)
    }
    /// Computes the base-ten logarithm, rounded half-even to `max(self.precision(), 20)` decimal places.
    ///
    /// # Errors
    /// Returns domain errors or `NoConvergence` when rounding cannot be certified within the budget.
    pub fn log10(&self) -> Result<Self, DecimalError> {
        self.log10_with_prec(self.precision().max(20))
    }
    /// Computes the base-ten logarithm, rounded half-even to `prec` decimal places.
    ///
    /// # Errors
    /// Returns domain errors or `NoConvergence` when rounding cannot be certified within the budget.
    ///
    /// # Panics
    /// Panics with `InvalidPrecision` for negative precision.
    pub fn log10_with_prec(&self, prec: isize) -> Result<Self, DecimalError> {
        self.certified_log(prec, 10)
    }
    fn certified_log(&self, prec: isize, base: i64) -> Result<Self, DecimalError> {
        let p = precision(prec);
        if self.sign() <= 0 {
            return Err(DecimalError::new(
                ErrorKind::InvalidLog,
                "logarithm of non-positive value",
            ));
        }
        if self.scale > MAX_PARSED_PRECISION || p > MAX_PARSED_PRECISION {
            return Err(no_convergence());
        }
        let r = self.big_rat();
        if r.is_one() {
            return Ok(Self::new_with_prec(0, prec));
        }
        if base == 2
            && r.numer().trailing_zeros() == Some(r.numer().bits() - 1)
            && r.denom().trailing_zeros() == Some(r.denom().bits() - 1)
        {
            return Ok(Self::new_with_append_prec(
                r.numer().bits() as i64 - r.denom().bits() as i64,
                prec,
            ));
        }
        if base == 10 {
            let n = r.numer().to_string();
            let d = r.denom().to_string();
            let power_ten =
                |s: &str| s.starts_with('1') && s.as_bytes()[1..].iter().all(|b| *b == b'0');
            if power_ten(&n) && power_ten(&d) {
                return Ok(Self::new_with_append_prec(
                    n.len() as i64 - d.len() as i64,
                    prec,
                ));
            }
        }
        let (arg, reciprocal) = reduce(r.numer(), r.denom(), base);
        let base_arg = if base != 0 {
            Some(binary_mantissa(&BigInt::from(base), &BigInt::one()))
        } else {
            None
        };
        certify(p, 0, |scale, bits| {
            let mut ln2 = None;
            let mut value = natural_bounds(&arg, scale, bits, &mut ln2)?;
            if reciprocal {
                value = value.mul_int(-1);
            }
            if let Some(ref b) = base_arg {
                value = value.divide(&natural_bounds(b, scale, bits, &mut ln2)?, scale);
            }
            Ok(value)
        })
    }
}
struct Argument {
    num: BigInt,
    den: BigInt,
    exponent: i64,
}
fn binary_mantissa(num: &BigInt, den: &BigInt) -> Argument {
    let (mut n, mut d) = (num.clone(), den.clone());
    let mut k = n.bits() as i64 - d.bits() as i64;
    if k >= 0 {
        d <<= k as usize;
    } else {
        n <<= (-k) as usize;
    }
    if n < d {
        n <<= 1usize;
        k -= 1;
    }
    Argument {
        num: n,
        den: d,
        exponent: k,
    }
}
fn reduce(num: &BigInt, den: &BigInt, base: i64) -> (Argument, bool) {
    let mut a = binary_mantissa(num, den);
    if a.num == a.den {
        return (a, false);
    }
    let mut reciprocal = base == 0 && a.exponent == -1;
    if base != 0 || (a.exponent != 0 && a.exponent != -1) {
        let (n, d) = if a.den.bits() > 64 {
            let shift = (a.den.bits() - 64) as usize;
            (&a.num >> shift, (&a.den >> shift) + 1)
        } else {
            (a.num.clone(), a.den.clone())
        };
        reciprocal = &n * &n > ((&d * &d) << 1usize);
    }
    if reciprocal {
        let old = a.num;
        a.num = a.den << 1usize;
        a.den = old;
        a.exponent = -a.exponent - 1;
    }
    (a, reciprocal)
}
fn mantissa_bounds(
    num: &BigInt,
    den: &BigInt,
    scale: &BigInt,
    max_terms: usize,
) -> Result<Interval, DecimalError> {
    if num == den {
        return Ok(Interval {
            lo: BigInt::zero(),
            hi: BigInt::zero(),
        });
    }
    let z = Interval::fraction(&(num - den), &(num + den), scale);
    let z2 = z.mul_positive(&z, scale);
    let mut power = z;
    let (mut lo, mut hi) = (BigInt::zero(), BigInt::zero());
    for j in 0..max_terms {
        let odd = BigInt::from(2 * j + 1);
        lo += floor_div(&power.lo, &odd);
        hi += ceil_div(&power.hi, &odd);
        let next = power.mul_positive(&z2, scale);
        if next.hi <= BigInt::from(2) {
            let tail = ceil_div(&(next.hi * 3), &BigInt::from(2 * j + 3));
            return Ok(Interval {
                lo: lo << 1usize,
                hi: (hi << 1usize) + tail,
            });
        }
        power = next;
    }
    Err(no_convergence())
}
fn natural_bounds(
    a: &Argument,
    scale: &BigInt,
    bits: usize,
    ln2: &mut Option<Interval>,
) -> Result<Interval, DecimalError> {
    let mut value = mantissa_bounds(&a.num, &a.den, scale, bits)?;
    if a.exponent != 0 {
        if ln2.is_none() {
            *ln2 = Some(mantissa_bounds(
                &BigInt::from(2),
                &BigInt::one(),
                scale,
                bits,
            )?);
        }
        let extra = ln2.as_ref().unwrap().mul_int(a.exponent);
        value.lo += extra.lo;
        value.hi += extra.hi;
    }
    Ok(value)
}
