use rs_decimal::{BigInt, BigRational, Decimal, DecimalError, ErrorKind, RoundingMode as M};
fn rat(d: &Decimal) -> BigRational {
    BigRational::new(d.big_int(), BigInt::from(10).pow(d.precision() as u32))
}

#[test]
fn exact_arithmetic_and_remainders_obey_rational_identities() {
    for a in -30..=30 {
        for b in -20..=20 {
            let x = Decimal::new_with_prec(a, 2);
            let y = Decimal::new_with_prec(b, 1);
            assert_eq!(rat(&x.add(&y)), rat(&x) + rat(&y));
            assert_eq!(rat(&x.sub(&y)), rat(&x) - rat(&y));
            assert_eq!(rat(&x.mul_exact(&y)), rat(&x) * rat(&y));
            if b != 0 {
                let (q, r) = x.quo_rem(&y);
                assert_eq!(rat(&x), rat(&q) * rat(&y) + rat(&r));
                assert_eq!(q.precision(), 0);
                assert_eq!(r.precision(), 2);
                assert!(r.abs() < y.abs());
                assert!(r.is_zero() || r.sign() == x.sign());
            }
        }
    }
}
#[test]
fn direct_rounding_default_scale_and_negative_divisor() {
    let x = Decimal::must_from_string("1.23");
    let y = Decimal::must_from_string("2.34");
    assert_eq!(x.mul(&y, M::HalfEven).string(), "2.88");
    assert_eq!(x.mul_exact(&y).string(), "2.8782");
    assert_eq!(x.mul_with_prec(&y, 3, M::HalfEven).string(), "2.878");
    assert_eq!(
        Decimal::new(1)
            .quo_with_prec(&Decimal::new(-8), 2, M::HalfEven)
            .string(),
        "-0.12"
    );
    assert_eq!(x.add_raw(1).string(), "1.24");
    assert_eq!(x.sub_raw(1).string(), "1.22");
    assert!(std::panic::catch_unwind(|| x.safe_sub(&y)).is_err());
    assert!(Decimal::default().must_non_negative().is_zero());
}
#[test]
fn powers_round_once_and_preserve_scale() {
    assert_eq!(Decimal::new(0).power(0), Decimal::new(1));
    assert_eq!(Decimal::new(2).power(-1), Decimal::new(0));
    assert_eq!(
        Decimal::must_from_string("2.00").power(-1).big_int(),
        BigInt::from(50)
    );
    assert_eq!(Decimal::new(-1).power(i64::MIN), Decimal::new(1));
    for (d, n, kind) in [
        (Decimal::new(0), -1, ErrorKind::DivideByZero),
        (Decimal::new(2), i64::MIN, ErrorKind::Overflow),
    ] {
        let err = std::panic::catch_unwind(|| d.power(n)).unwrap_err();
        assert!(err.downcast_ref::<DecimalError>().unwrap().matches(kind));
    }
}
#[test]
fn extreme_scale_fast_paths_and_checked_overflow() {
    let a = Decimal::new_with_prec(3, isize::MAX);
    let b = Decimal::new(2);
    assert_eq!(a.mul(&b, M::Unnecessary).big_int(), BigInt::from(6));
    assert_eq!(
        a.mul_with_prec(&a, 2, M::HalfEven).big_int(),
        BigInt::from(0)
    );
    assert_eq!(
        a.neg().mul_with_prec(&a, 2, M::Up).big_int(),
        BigInt::from(-1)
    );
    assert_eq!(
        Decimal::new(0)
            .quo_with_prec(&a, isize::MAX, M::Unnecessary)
            .precision(),
        isize::MAX
    );
    for err in [
        std::panic::catch_unwind(|| a.mul_exact(&b)).unwrap_err(),
        std::panic::catch_unwind(|| b.quo_with_prec(&a, isize::MAX, M::Down)).unwrap_err(),
    ] {
        assert!(
            err.downcast_ref::<DecimalError>()
                .unwrap()
                .matches(ErrorKind::Overflow)
        );
    }
    assert_eq!(a.int_part(), BigInt::from(0));
    assert!(!a.is_integer());
    assert!(a.has_fraction());
    assert_eq!(
        Decimal::must_from_string("-12.34").remainder(),
        (BigInt::from(-12), BigInt::from(-34))
    );
}

#[test]
fn zero_divisor_is_checked_before_extreme_scale_alignment() {
    let divisor = Decimal::new_with_prec(0, isize::MAX);
    let a = Decimal::new(1);
    for op in [0, 1, 2] {
        let err = std::panic::catch_unwind(|| match op {
            0 => a.quo_rem(&divisor).1,
            1 => a.modulo(&divisor),
            _ => a.r#mod(&divisor),
        })
        .unwrap_err();
        assert!(
            err.downcast_ref::<DecimalError>()
                .unwrap()
                .matches(ErrorKind::DivideByZero)
        );
    }
    assert_eq!(divisor.precision(), isize::MAX);
    assert_eq!(a.big_int(), BigInt::from(1));
}
