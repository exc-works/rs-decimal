//! Bounded Ziv refinement accepts agreement of enclosure endpoints only.
#![allow(dead_code)] // Used by public logarithm/exponential functions in PR 09.
use crate::{
    Decimal, DecimalError, ErrorKind,
    integer::{MAX_PARSED_PRECISION, pow10},
    interval::Interval,
};
use num_bigint::BigInt;

pub(crate) fn no_convergence() -> DecimalError {
    DecimalError::new(
        ErrorKind::NoConvergence,
        "cannot certify rounding within precision budget",
    )
}
fn decimal_bits(p: usize) -> usize {
    (p * 3322).div_ceil(1000)
}
pub(crate) fn certify(
    p: usize,
    extra: usize,
    mut enclose: impl FnMut(&BigInt, usize) -> Result<Interval, DecimalError>,
) -> Result<Decimal, DecimalError> {
    if p > MAX_PARSED_PRECISION || extra > MAX_PARSED_PRECISION - p {
        return Err(no_convergence());
    }
    let base_bits = decimal_bits(p + extra);
    let max_bits = decimal_bits(MAX_PARSED_PRECISION);
    let multiplier = pow10(p);
    let mut guard = 64;
    while guard <= 8192 {
        let bits = base_bits + guard;
        if bits > max_bits {
            break;
        }
        let scale = BigInt::from(1) << bits;
        let bounds = enclose(&scale, bits)?;
        if let Some(c) = bounds.rounded(&scale, &multiplier) {
            return Ok(Decimal::from_parts(c, p));
        }
        guard *= 2;
    }
    Err(no_convergence())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn retries_until_interval_proves_rounding() {
        let mut calls = Vec::new();
        let got = certify(0, 0, |s, bits| {
            calls.push(bits);
            Ok(if bits == 64 {
                Interval {
                    lo: s / 2 - 1,
                    hi: s / 2 + 1,
                }
            } else {
                Interval {
                    lo: s / 4,
                    hi: s / 4 + 1,
                }
            })
        })
        .unwrap();
        assert_eq!(calls, vec![64, 128]);
        assert_eq!(got, Decimal::new(0));
        assert!(!got.is_nil());
        let mut count = 0;
        let err = certify(0, 0, |s, _| {
            count += 1;
            Ok(Interval {
                lo: s / 2 - 1,
                hi: s / 2 + 1,
            })
        })
        .unwrap_err();
        assert_eq!(count, 8);
        assert!(err.matches(ErrorKind::NoConvergence));
    }
    #[test]
    fn budgets_and_evaluation_errors_never_return_approximations() {
        for (p, extra) in [(usize::MAX, 0), (0, usize::MAX), (131072, 0), (131071, 1)] {
            assert!(
                certify(p, extra, |_, _| panic!(
                    "budget must reject before enclosure"
                ))
                .unwrap_err()
                .matches(ErrorKind::NoConvergence)
            );
        }
        let mut calls = 0;
        let err = certify(20, 0, |_, _| {
            calls += 1;
            Err(DecimalError::new(ErrorKind::InvalidLog, "test"))
        })
        .unwrap_err();
        assert_eq!(calls, 1);
        assert!(err.matches(ErrorKind::InvalidLog));
    }
}
