//! Outward fixed-point enclosures. Endpoints own their integers; inputs are borrowed.
use crate::{
    RoundingMode,
    integer::{ceil_div, floor_div},
    rounding::rounded_div,
};
use num_bigint::BigInt;
use num_traits::{Signed, Zero};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Interval {
    pub lo: BigInt,
    pub hi: BigInt,
}
impl Interval {
    pub fn fraction(num: &BigInt, den: &BigInt, scale: &BigInt) -> Self {
        assert!(den.is_positive() && scale.is_positive());
        let n = num * scale;
        Self {
            lo: floor_div(&n, den),
            hi: ceil_div(&n, den),
        }
    }
    pub fn mul_positive(&self, other: &Self, scale: &BigInt) -> Self {
        assert!(!self.lo.is_negative() && !other.lo.is_negative() && scale.is_positive());
        Self {
            lo: floor_div(&(&self.lo * &other.lo), scale),
            hi: ceil_div(&(&self.hi * &other.hi), scale),
        }
    }
    pub fn divide(&self, other: &Self, scale: &BigInt) -> Self {
        assert!(other.lo.is_positive() && scale.is_positive());
        let lower_den = if self.lo.is_negative() {
            &other.lo
        } else {
            &other.hi
        };
        let upper_den = if self.hi <= BigInt::zero() {
            &other.hi
        } else {
            &other.lo
        };
        Self {
            lo: floor_div(&(&self.lo * scale), lower_den),
            hi: ceil_div(&(&self.hi * scale), upper_den),
        }
    }
    pub fn mul_int(&self, k: i64) -> Self {
        let (lo, hi) = if k < 0 {
            (&self.hi, &self.lo)
        } else {
            (&self.lo, &self.hi)
        };
        Self {
            lo: lo * k,
            hi: hi * k,
        }
    }
    pub fn rounded(&self, scale: &BigInt, multiplier: &BigInt) -> Option<BigInt> {
        assert!(self.lo <= self.hi && scale.is_positive());
        let lo = rounded_div(&(&self.lo * multiplier), scale, RoundingMode::HalfEven);
        let hi = rounded_div(&(&self.hi * multiplier), scale, RoundingMode::HalfEven);
        (lo == hi).then_some(lo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_rational::BigRational as Rat;
    fn rat(n: i64, d: i64) -> Rat {
        Rat::new(n.into(), d.into())
    }
    fn contains(a: &Interval, scale: &BigInt, x: &Rat) {
        assert!(Rat::new(a.lo.clone(), scale.clone()) <= *x);
        assert!(Rat::new(a.hi.clone(), scale.clone()) >= *x);
    }
    #[test]
    fn outward_arithmetic_contains_exact_rationals() {
        for scale in [3, 16, 256] {
            let s = BigInt::from(scale);
            for n in -17..=17 {
                for den in 1..=13 {
                    let x = rat(n, den);
                    let a = Interval::fraction(x.numer(), x.denom(), &s);
                    let before = a.clone();
                    contains(&a, &s, &x);
                    for k in [-7, 0, 3] {
                        contains(&a.mul_int(k), &s, &(&x * BigInt::from(k)));
                    }
                    for m in 1..=9 {
                        let y = rat(m, 2);
                        let b = Interval::fraction(y.numer(), y.denom(), &s);
                        contains(&a.divide(&b, &s), &s, &(&x / &y));
                        if n >= 0 {
                            contains(&a.mul_positive(&b, &s), &s, &(&x * &y));
                        }
                    }
                    assert_eq!(a, before);
                }
            }
            // Division must also enclose an entire interval that crosses zero.
            let a = Interval {
                lo: (-5).into(),
                hi: 7.into(),
            };
            let b = Interval {
                lo: 2.into(),
                hi: 9.into(),
            };
            let q = a.divide(&b, &s);
            for n in -5..=7 {
                for d in 2..=9 {
                    contains(&q, &s, &rat(n, d));
                }
            }
        }
    }
    #[test]
    fn endpoint_rounding_rejects_ambiguous_cells() {
        let scale = BigInt::from(100);
        for (lo, hi, want) in [
            (149, 151, None),
            (-151, -149, None),
            (150, 150, Some(2)),
            (250, 250, Some(2)),
            (-250, -250, Some(-2)),
            (-49, 49, Some(0)),
        ] {
            let a = Interval {
                lo: lo.into(),
                hi: hi.into(),
            };
            assert_eq!(a.rounded(&scale, &1.into()), want.map(BigInt::from));
        }
    }
}
