use rs_decimal::{BigInt, BigRational, Decimal, ErrorKind, RoundingMode as M};
fn rat(d: &Decimal) -> BigRational {
    BigRational::new(d.big_int(), BigInt::from(10).pow(d.precision() as u32))
}

#[test]
fn exact_arithmetic_and_remainders_obey_rational_identities() {
    for a in -30..=30 {
        for b in -20..=20 {
            let x = Decimal::new_with_prec(a, 2).unwrap();
            let y = Decimal::new_with_prec(b, 1).unwrap();
            assert_eq!(rat(&x.add(&y)), rat(&x) + rat(&y));
            assert_eq!(rat(&x.sub(&y)), rat(&x) - rat(&y));
            assert_eq!(rat(&x.mul_exact(&y).unwrap()), rat(&x) * rat(&y));
            if b != 0 {
                let (q, r) = x.quo_rem(&y).unwrap();
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
    assert_eq!(x.mul(&y, M::HalfEven).unwrap().string(), "2.88");
    assert_eq!(x.mul_exact(&y).unwrap().string(), "2.8782");
    assert_eq!(
        x.mul_with_prec(&y, 3, M::HalfEven).unwrap().string(),
        "2.878"
    );
    assert_eq!(
        Decimal::new(1)
            .quo_with_prec(&Decimal::new(-8), 2, M::HalfEven)
            .unwrap()
            .string(),
        "-0.12"
    );
    assert_eq!(x.add_raw(1).string(), "1.24");
    assert_eq!(x.sub_raw(1).string(), "1.22");
    assert!(x.safe_sub(&y).is_err());
    assert!(Decimal::default().must_non_negative().is_zero());
}
#[test]
fn powers_round_once_and_preserve_scale() {
    assert_eq!(Decimal::new(0).power(0).unwrap(), Decimal::new(1));
    assert_eq!(Decimal::new(2).power(-1).unwrap(), Decimal::new(0));
    assert_eq!(
        Decimal::must_from_string("2.00")
            .power(-1)
            .unwrap()
            .big_int(),
        BigInt::from(50)
    );
    assert_eq!(Decimal::new(-1).power(i64::MIN).unwrap(), Decimal::new(1));
    for (d, n, kind) in [
        (Decimal::new(0), -1, ErrorKind::DivideByZero),
        (Decimal::new(2), i64::MIN, ErrorKind::Overflow),
    ] {
        let err = d.power(n).unwrap_err();
        assert!(err.matches(kind));
    }
}
#[test]
fn extreme_scale_fast_paths_and_checked_overflow() {
    let a = Decimal::new_with_prec(3, isize::MAX).unwrap();
    let b = Decimal::new(2);
    assert_eq!(
        a.mul(&b, M::Unnecessary).unwrap().big_int(),
        BigInt::from(6)
    );
    assert_eq!(
        a.mul_with_prec(&a, 2, M::HalfEven).unwrap().big_int(),
        BigInt::from(0)
    );
    assert_eq!(
        a.neg().mul_with_prec(&a, 2, M::Up).unwrap().big_int(),
        BigInt::from(-1)
    );
    assert_eq!(
        Decimal::new(0)
            .quo_with_prec(&a, isize::MAX, M::Unnecessary)
            .unwrap()
            .precision(),
        isize::MAX
    );
    // The fixed i32 scale cap itself is valid on both pointer widths.
    let at_cap = Decimal::new_with_prec(3, i32::MAX as isize)
        .unwrap()
        .mul_exact(&b)
        .unwrap();
    assert_eq!(at_cap.big_int(), BigInt::from(6));
    assert_eq!(at_cap.precision(), i32::MAX as isize);
    for err in [
        a.mul_exact(&Decimal::new_with_prec(2, 1).unwrap())
            .unwrap_err(),
        b.quo_with_prec(&a, isize::MAX, M::Down).unwrap_err(),
    ] {
        assert!(err.matches(ErrorKind::Overflow));
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
    let divisor = Decimal::new_with_prec(0, isize::MAX).unwrap();
    let a = Decimal::new(1);
    for op in [0, 1, 2] {
        let err = match op {
            0 => a.quo_rem(&divisor).map(|(_, remainder)| remainder),
            1 => a.modulo(&divisor),
            _ => a.r#mod(&divisor),
        }
        .unwrap_err();
        assert!(err.matches(ErrorKind::DivideByZero));
    }
    assert_eq!(divisor.precision(), isize::MAX);
    assert_eq!(a.big_int(), BigInt::from(1));
}

#[test]
fn aligned_add_sub_preserve_full_state_and_inputs() {
    let mut values = vec![Decimal::default()];
    for scale in [0, 2, 5] {
        for c in [-100001, -123, 0, 123, 100001] {
            values.push(Decimal::new_with_prec(c, scale).unwrap());
        }
        let c: BigInt = "99999999999999999999999999999999999999999999999999"
            .parse()
            .unwrap();
        values.push(Decimal::new_from_big_int_with_prec(&c, scale).unwrap());
        values.push(Decimal::new_from_big_int_with_prec(&-c, scale).unwrap());
    }
    let state = |d: &Decimal| (d.big_int(), d.precision(), d.is_nil());
    for a in &values {
        for b in &values {
            let before = (state(a), state(b));
            for (result, expected) in [(a.add(b), rat(a) + rat(b)), (a.sub(b), rat(a) - rat(b))] {
                assert_eq!(rat(&result), expected);
                assert_eq!(result.precision(), a.precision().max(b.precision()));
                assert!(!result.is_nil());
            }
            assert_eq!((state(a), state(b)), before);
        }
    }
}

#[test]
fn safe_arithmetic_keeps_state_and_returns_errors() {
    let nil = Decimal::default();
    let zero = Decimal::new_with_prec(0, 5).unwrap();
    for result in [nil.safe_add(&zero).unwrap(), nil.safe_sub(&zero).unwrap()] {
        assert!(result.is_zero());
        assert!(!result.is_nil());
        assert_eq!(result.precision(), 5);
    }
    assert!(nil.is_nil());
    for result in [
        Decimal::new(2).safe_add(&Decimal::new(1)).unwrap(),
        Decimal::new(4).safe_sub(&Decimal::new(1)).unwrap(),
    ] {
        assert_eq!(result.big_int(), BigInt::from(3));
    }
    for failure in [
        Decimal::new(-2).safe_add(&Decimal::new(1)),
        Decimal::new(1).safe_sub(&Decimal::new(2)),
    ] {
        let error = failure.unwrap_err();
        assert!(error.matches(ErrorKind::InvalidArgument));
        assert_eq!(error.to_string(), "Negative value");
    }
}

#[test]
fn division_shift_branches_preserve_inputs_and_exactness() {
    for (a_scale, b_scale, p) in [(0, 2, 2), (2, 0, 2), (5, 0, 2)] {
        for sign in [-1, 1] {
            let a = Decimal::new_with_prec(12345, a_scale).unwrap();
            let b = Decimal::new_with_prec(sign * 8, b_scale).unwrap();
            for mode in [
                M::Down,
                M::Up,
                M::Ceiling,
                M::HalfUp,
                M::HalfDown,
                M::HalfEven,
            ] {
                let result = a.quo_with_prec(&b, p, mode).unwrap();
                let expected =
                    Decimal::new_from_big_rat_with_prec(&(rat(&a) / rat(&b)), p, mode).unwrap();
                assert_eq!(result.big_int(), expected.big_int());
                assert_eq!(result.precision(), p);
                assert!(!result.is_nil());
            }
            assert_eq!(a.big_int(), BigInt::from(12345));
            assert_eq!(b.big_int(), BigInt::from(sign * 8));
        }
    }
    for (a_scale, b_scale, p) in [(0, 2, 2), (2, 0, 2), (3, 0, 2)] {
        let a = Decimal::new_with_prec(8000, a_scale).unwrap();
        let result = a
            .quo_with_prec(
                &Decimal::new_with_prec(-8, b_scale).unwrap(),
                p,
                M::Unnecessary,
            )
            .unwrap();
        assert_eq!(
            rat(&result),
            rat(&a) / rat(&Decimal::new_with_prec(-8, b_scale).unwrap())
        );
        assert_eq!(result.precision(), p);
    }
}
