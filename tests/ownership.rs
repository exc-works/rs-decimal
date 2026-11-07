use rs_decimal::{BigInt, Decimal, DecimalError, ErrorKind, RoundingMode as M};

fn state(d: &Decimal) -> (BigInt, usize, bool) {
    (d.big_int(), d.scale(), d.is_nil())
}

#[test]
fn coefficient_access_and_construction_keep_explicit_state() {
    let coefficient: BigInt = "1234567890123456789012345678901234567890".parse().unwrap();
    let d = Decimal::try_from_coefficient(coefficient.clone(), 5).unwrap();
    assert_eq!(d.coefficient(), &coefficient);
    assert!(std::ptr::eq(d.coefficient(), d.coefficient()));
    assert_eq!(d.scale(), 5);
    assert!(!d.is_nil());
    assert_eq!(d.into_coefficient(), coefficient);
    let nil = Decimal::default();
    assert_eq!(nil.coefficient(), &BigInt::from(0));
    assert!(nil.is_nil());
    assert_eq!(nil.into_coefficient(), BigInt::from(0));
    let zero = Decimal::try_from_coefficient(BigInt::from(0), isize::MAX as usize).unwrap();
    assert_eq!(zero.scale(), isize::MAX as usize);
    assert!(!zero.is_nil());
    let tiny = Decimal::try_from_coefficient(BigInt::from(1), isize::MAX as usize).unwrap();
    assert_eq!(tiny.coefficient(), &BigInt::from(1));
    for scale in [isize::MAX as usize + 1, usize::MAX] {
        assert!(
            Decimal::try_from_coefficient(BigInt::from(1), scale)
                .unwrap_err()
                .matches(ErrorKind::Overflow)
        );
    }
}

#[test]
fn borrowed_extrema_select_identity_and_first_representation_on_ties() {
    let nil = Decimal::default();
    let zero = Decimal::new_with_prec(0, 3);
    let a = Decimal::new_with_prec(100, 2);
    let b = Decimal::new(1);
    for (left, right) in [(&nil, &zero), (&zero, &nil), (&a, &b), (&b, &a)] {
        assert!(std::ptr::eq(left.max_ref(right), left));
        assert!(std::ptr::eq(left.min_ref(right), left));
    }
    let negative = Decimal::new(-1);
    assert!(std::ptr::eq(negative.max_ref(&a), &a));
    assert!(std::ptr::eq(a.min_ref(&negative), &negative));
}

#[test]
fn consuming_arithmetic_matches_borrowed_full_state() {
    let mut values = vec![Decimal::default()];
    for scale in [0, 2, 5] {
        for c in [-999999, -123, 0, 123, 999999] {
            values.push(Decimal::new_with_prec(c, scale));
        }
        let c: BigInt = "99999999999999999999999999999999999999999999999999"
            .parse()
            .unwrap();
        values.push(Decimal::try_from_coefficient(c.clone(), scale as usize).unwrap());
        values.push(Decimal::try_from_coefficient(-c, scale as usize).unwrap());
    }
    for a in &values {
        for b in &values {
            let before = state(b);
            assert_eq!(state(&a.clone().add_owned(b)), state(&a.add(b)));
            assert_eq!(state(&a.clone().sub_owned(b)), state(&a.sub(b)));
            assert_eq!(state(b), before);
        }
    }
    let zero = Decimal::new_with_prec(0, isize::MAX);
    for result in [
        Decimal::default().add_owned(&zero),
        Decimal::default().sub_owned(&zero),
    ] {
        assert_eq!(
            state(&result),
            (BigInt::from(0), isize::MAX as usize, false)
        );
    }
    let tiny = Decimal::new_with_prec(-3, isize::MAX);
    assert_eq!(state(&Decimal::default().add_owned(&tiny)), state(&tiny));
    assert_eq!(
        state(&tiny.clone().sub_owned(&Decimal::default())),
        state(&tiny)
    );
}

#[test]
fn consuming_rescale_matches_all_rounding_modes_and_failures() {
    for coefficient in -31..=31 {
        for scale in 0..=3 {
            let d = Decimal::new_with_prec(coefficient, scale);
            for target in 0..=4 {
                for mode in [
                    M::Down,
                    M::Up,
                    M::Ceiling,
                    M::HalfUp,
                    M::HalfDown,
                    M::HalfEven,
                    M::Unnecessary,
                ] {
                    let borrowed = std::panic::catch_unwind(|| d.rescale(target, mode));
                    let owned = std::panic::catch_unwind(|| d.clone().into_rescaled(target, mode));
                    match (borrowed, owned) {
                        (Ok(a), Ok(b)) => assert_eq!(state(&a), state(&b)),
                        (Err(a), Err(b)) => {
                            let a = a.downcast_ref::<DecimalError>().unwrap();
                            let b = b.downcast_ref::<DecimalError>().unwrap();
                            assert_eq!(a, b);
                            assert!(b.matches(ErrorKind::RoundUnnecessary));
                        }
                        _ => panic!("owned and borrowed outcomes differ"),
                    }
                }
            }
        }
    }
    for target in [0, 2, isize::MAX] {
        let result = Decimal::default().into_rescaled(target, M::Unnecessary);
        assert_eq!(state(&result), (BigInt::from(0), target as usize, false));
    }
    let err =
        std::panic::catch_unwind(|| Decimal::default().into_rescaled(-1, M::Down)).unwrap_err();
    assert!(
        err.downcast_ref::<DecimalError>()
            .unwrap()
            .matches(ErrorKind::InvalidPrecision)
    );
    let tiny = Decimal::new_with_prec(1, isize::MAX);
    assert_eq!(
        state(&tiny.clone().into_rescaled(isize::MAX, M::Unnecessary)),
        state(&tiny)
    );
}
