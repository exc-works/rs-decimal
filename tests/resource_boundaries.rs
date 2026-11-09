use rs_decimal::{BigInt, Decimal, ErrorKind, RoundingMode};

fn assert_parts(value: Decimal, coefficient: i64, scale: usize) {
    assert_eq!(value.coefficient(), &BigInt::from(coefficient));
    assert_eq!(value.scale(), scale);
    assert!(!value.is_nil());
}

#[test]
fn zero_construction_and_rational_conversion_do_not_expand_scale() {
    for zero in [
        Decimal::new_with_append_prec(0, isize::MAX).unwrap(),
        Decimal::new_from_uint_with_append_prec(0, isize::MAX).unwrap(),
    ] {
        assert_eq!(zero.big_rat().numer(), &BigInt::from(0));
        assert_eq!(zero.big_rat().denom(), &BigInt::from(1));
        assert_parts(zero, 0, isize::MAX as usize);
    }
    for invalid in [
        Decimal::new_with_append_prec(0, -1),
        Decimal::new_from_uint_with_append_prec(0, -1),
    ] {
        assert!(invalid.unwrap_err().matches(ErrorKind::InvalidPrecision));
    }
}

#[test]
fn tiny_values_round_without_expanding_native_maximum_scales() {
    for sign in [-1, 1] {
        let tiny = Decimal::new_with_prec(sign, isize::MAX).unwrap();
        for (mode, expected) in [
            (RoundingMode::Down, 0),
            (RoundingMode::Up, sign),
            (RoundingMode::Ceiling, i64::from(sign > 0)),
            (RoundingMode::HalfUp, 0),
            (RoundingMode::HalfDown, 0),
            (RoundingMode::HalfEven, 0),
        ] {
            assert_parts(tiny.rescale(2, mode).unwrap(), expected, 2);
            assert_parts(tiny.clone().into_rescaled(2, mode).unwrap(), expected, 2);
            // Both coefficients stay small even though the summed scale exceeds isize::MAX.
            let positive = Decimal::new_with_prec(1, isize::MAX).unwrap();
            assert_parts(
                tiny.mul(&positive, mode).unwrap(),
                expected,
                isize::MAX as usize,
            );
            assert_parts(tiny.mul_with_prec(&positive, 2, mode).unwrap(), expected, 2);
            for divisor in [-3i64, 3] {
                let expected = match mode {
                    RoundingMode::Up => sign * divisor.signum(),
                    RoundingMode::Ceiling => i64::from(sign * divisor > 0),
                    _ => 0,
                };
                assert_parts(
                    tiny.quo_with_prec(&Decimal::new(divisor), 2, mode).unwrap(),
                    expected,
                    2,
                );
            }
        }
        assert_parts(tiny.floor(), if sign < 0 { -1 } else { 0 }, 0);
        assert_parts(tiny.ceil(), if sign > 0 { 1 } else { 0 }, 0);
        assert_parts(tiny.truncate(), 0, 0);
        assert_parts(tiny.round(), 0, 0);
        for result in [
            tiny.rescale(0, RoundingMode::Unnecessary),
            tiny.clone().into_rescaled(0, RoundingMode::Unnecessary),
            tiny.mul(&tiny, RoundingMode::Unnecessary),
            tiny.quo_with_prec(&Decimal::new(-3), 0, RoundingMode::Unnecessary),
        ] {
            assert!(result.unwrap_err().matches(ErrorKind::RoundUnnecessary));
        }
        assert_eq!(tiny.coefficient(), &BigInt::from(sign));
        assert_eq!(tiny.scale(), isize::MAX as usize);
    }
}

#[test]
fn zero_dividend_does_not_expand_a_nonzero_divisor() {
    for dividend in [
        Decimal::default(),
        Decimal::new_with_prec(0, isize::MAX).unwrap(),
    ] {
        for divisor in [
            Decimal::new(-3),
            Decimal::new_with_prec(3, isize::MAX).unwrap(),
        ] {
            let scale = dividend.scale().max(divisor.scale());
            let (quotient, remainder) = dividend.quo_rem(&divisor).unwrap();
            assert_parts(quotient, 0, 0);
            assert_parts(remainder, 0, scale);
            assert_parts(dividend.modulo(&divisor).unwrap(), 0, scale);
        }
    }
    let zero = Decimal::new_with_prec(0, isize::MAX).unwrap();
    assert!(
        zero.quo_rem(&zero)
            .unwrap_err()
            .matches(ErrorKind::DivideByZero)
    );
}

#[test]
fn exact_fast_paths_keep_their_existing_budget_exceptions() {
    // Root identities precede the general root budget; log/exp limits precede identities.
    assert_parts(Decimal::new(0).sqrt_with_prec(131073).unwrap(), 0, 131073);
    assert_parts(
        Decimal::new_with_prec(1, isize::MAX)
            .unwrap()
            .approx_root_with_prec(1, 0)
            .unwrap(),
        0,
        0,
    );
    for result in [
        Decimal::new(1).ln_with_prec(131073),
        Decimal::new(0).exp_with_prec(131073),
    ] {
        assert!(result.unwrap_err().matches(ErrorKind::NoConvergence));
    }
    // Constructors do not inherit the parser's scale cap.
    assert_parts(Decimal::new_with_prec(1, 131073).unwrap(), 1, 131073);
    assert!("0e131073".parse::<Decimal>().is_err());
    assert!("0e-131073".parse::<Decimal>().is_err());
    assert_parts("0e131072".parse().unwrap(), 0, 0);
    assert_parts("-0e-131072".parse().unwrap(), 0, 0);
}

#[test]
fn certified_operations_reject_excessive_precision_before_work() {
    let two = Decimal::new(2);
    for result in [
        two.ln_with_prec(131073),
        two.exp_with_prec(131073),
        two.sqrt_with_prec(131073),
        two.approx_root_with_prec(131073, 1),
    ] {
        assert!(result.unwrap_err().matches(ErrorKind::NoConvergence));
    }
    assert!("1e131073".parse::<Decimal>().is_err());
    assert!("1e-131073".parse::<Decimal>().is_err());
}

#[test]
fn rejected_binary_scale_preserves_representation() {
    let mut value = Decimal::must_from_string("12.3400");
    let before = (value.big_int(), value.precision(), value.is_nil());
    for scale in [131073u32, u32::MAX] {
        let mut wire = scale.to_be_bytes().to_vec();
        wire.extend_from_slice(&[2, 1]);
        assert!(value.unmarshal_binary(&wire).is_err());
        assert_eq!((value.big_int(), value.precision(), value.is_nil()), before);
    }
}
