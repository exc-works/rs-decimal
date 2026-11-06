use num_integer::Integer;
use num_traits::{Signed, Zero};
use rs_decimal::{BigInt, BigRational, Decimal, DecimalError, ErrorKind, RoundingMode as M};

// Independent reference: choose between adjacent integers using exact distances.
fn oracle(r: BigRational, mode: M) -> Option<BigInt> {
    let lo = r.numer().div_floor(r.denom());
    let distance = &r - BigRational::from_integer(lo.clone());
    if distance.is_zero() {
        return Some(lo);
    }
    let hi = &lo + 1;
    if mode == M::Unnecessary {
        return None;
    }
    let positive = r.is_positive();
    match mode {
        M::Down => Some(if positive { lo } else { hi }),
        M::Up => Some(if positive { hi } else { lo }),
        M::Ceiling => Some(hi),
        _ => {
            let cmp = distance.cmp(&BigRational::new(1.into(), 2.into()));
            Some(if cmp.is_lt() {
                lo
            } else if cmp.is_gt() {
                hi
            } else {
                match mode {
                    M::HalfEven => {
                        if lo.is_even() {
                            lo
                        } else {
                            hi
                        }
                    }
                    M::HalfUp => {
                        if positive {
                            hi
                        } else {
                            lo
                        }
                    }
                    M::HalfDown => {
                        if positive {
                            lo
                        } else {
                            hi
                        }
                    }
                    _ => unreachable!(),
                }
            })
        }
    }
}

#[test]
fn all_modes_match_exact_rational_neighbors() {
    for n in -101..=101 {
        for scale in 0..=3 {
            for target in 0..=3 {
                for raw in 0..=6 {
                    let mode = M::from_raw(raw);
                    let d = Decimal::new_with_prec(n, scale);
                    let r = BigRational::new(
                        BigInt::from(n) * BigInt::from(10).pow(target as u32),
                        BigInt::from(10).pow(scale as u32),
                    );
                    let want = oracle(r, mode);
                    let got = std::panic::catch_unwind(|| d.rescale(target, mode));
                    match want {
                        Some(c) => {
                            let got = got.unwrap();
                            assert_eq!(got.big_int(), c);
                            assert_eq!(got.precision(), target);
                            assert!(!got.is_nil());
                        }
                        None => assert!(
                            got.unwrap_err()
                                .downcast_ref::<DecimalError>()
                                .unwrap()
                                .matches(ErrorKind::RoundUnnecessary)
                        ),
                    }
                    assert_eq!(d.big_int(), BigInt::from(n));
                    assert_eq!(d.precision(), scale);
                }
            }
        }
    }
}

#[test]
fn floor_ceil_shift_and_significant_figures() {
    let d = Decimal::must_from_string("-12.345");
    assert_eq!(d.floor_with_prec(2).string(), "-12.35");
    assert_eq!(d.ceil_with_prec(2).string(), "-12.34");
    assert_eq!(d.shift(4).string(), "-123450");
    assert_eq!(d.shift(-2).string(), "-0.12345");
    assert_eq!(
        Decimal::must_from_string("12345")
            .significant_figures(3, M::HalfEven)
            .string(),
        "12300"
    );
    assert_eq!(
        Decimal::must_from_string("0.0012300")
            .significant_figures(2, M::HalfEven)
            .string(),
        "0.0012"
    );
    assert_eq!(
        Decimal::must_from_string("1.20")
            .significant_figures(9, M::HalfEven)
            .precision(),
        2
    );
    assert!(std::panic::catch_unwind(|| M::from_raw(7)).is_err());
    let err = std::panic::catch_unwind(|| Decimal::new(1).shift(isize::MIN)).unwrap_err();
    assert!(
        err.downcast_ref::<DecimalError>()
            .unwrap()
            .matches(ErrorKind::Overflow)
    );
}

#[test]
fn zero_and_trailing_zero_states() {
    let nil = Decimal::default();
    assert!(!nil.round().is_nil());
    assert!(!nil.shift(0).is_nil());
    let zero = Decimal::new_with_prec(0, 8);
    assert_eq!(zero.rescale(3, M::Unnecessary).precision(), 3);
    assert_eq!(zero.significant_figures(2, M::Down).precision(), 8);
    assert_eq!(zero.strip_trailing_zeros().precision(), 0);
    for s in [
        "123.45000",
        "1000.0000000000000000000000",
        "-0.0000000000010000000000000000000000",
    ] {
        let d = Decimal::must_from_string(s);
        let stripped = d.strip_trailing_zeros();
        assert_eq!(d, stripped);
        assert_eq!(
            stripped.precision(),
            Decimal::must_from_string(&d.string()).precision()
        );
    }
    let d = Decimal::new_with_prec(1000, isize::MAX);
    assert_eq!(d.strip_trailing_zeros().precision(), isize::MAX - 3);
}
