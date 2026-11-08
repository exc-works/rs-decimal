use rs_decimal::{
    BigInt, BigRational, Decimal, DecimalError, ErrorKind, GoFormat, RoundingMode as M,
};

fn assert_error<T: std::fmt::Debug>(result: Result<T, DecimalError>, kind: ErrorKind) {
    assert_eq!(result.unwrap_err().kinds(), &[kind]);
}

#[test]
fn negative_precision_returns_errors_before_domain_checks_and_zero_fast_paths() {
    for precision in [-1, isize::MIN] {
        for value in [-1, 0, 1] {
            let coefficient = BigInt::from(value);
            for result in [
                Decimal::new_with_prec(value, precision),
                Decimal::new_from_int64(value, precision),
                Decimal::new_from_big_int_with_prec(&coefficient, precision),
                Decimal::new_with_append_prec(value, precision),
                Decimal::new_from_uint64(value.unsigned_abs(), precision),
                Decimal::new_from_uint_with_append_prec(value.unsigned_abs(), precision),
            ] {
                assert_error(result, ErrorKind::InvalidPrecision);
            }
        }
        for value in [Decimal::default(), Decimal::new(-1), Decimal::new(1)] {
            let before = (value.big_int(), value.scale(), value.is_nil());
            for result in [
                value.rescale(precision, M::Unnecessary),
                value.rescale_down(precision),
                value.clone().into_rescaled(precision, M::Unnecessary),
                value.truncate_with_prec(precision),
                value.round_with_prec(precision),
                value.floor_with_prec(precision),
                value.ceil_with_prec(precision),
                value.mul_with_prec(&Decimal::new(0), precision, M::Unnecessary),
                value.quo_with_prec(&Decimal::new(0), precision, M::Unnecessary),
                value.sqrt_with_prec(precision),
                value.approx_root_with_prec(0, precision),
                value.ln_with_prec(precision),
                value.log2_with_prec(precision),
                value.log10_with_prec(precision),
                value.exp_with_prec(precision),
                Decimal::new_from_big_rat_with_prec(&value.big_rat(), precision, M::Unnecessary),
            ] {
                assert_error(result, ErrorKind::InvalidPrecision);
            }
            assert_eq!((value.big_int(), value.scale(), value.is_nil()), before);
        }
    }
}

#[test]
fn division_and_exact_rounding_failures_are_returned_and_leave_inputs_unchanged() {
    let a: Decimal = "1.2300".parse().unwrap();
    let zero = Decimal::new_with_prec(0, isize::MAX).unwrap();
    for result in [
        a.quo(&zero, M::Down),
        a.quo_down(&zero),
        a.quo_with_prec(&zero, 2, M::Unnecessary),
        a.modulo(&zero),
        a.r#mod(&zero),
        a.quo_rem(&zero).map(|(q, _)| q),
        zero.power(-1),
    ] {
        assert_error(result, ErrorKind::DivideByZero);
    }
    let fraction = BigRational::new(1.into(), 3.into());
    for result in [
        a.rescale(1, M::Unnecessary),
        a.clone().into_rescaled(1, M::Unnecessary),
        a.mul(&Decimal::must_from_string("1.2345"), M::Unnecessary),
        a.mul_with_prec(&Decimal::new(1), 1, M::Unnecessary),
        a.quo_with_prec(&Decimal::new(2), 2, M::Unnecessary),
        a.significant_figures(2, M::Unnecessary),
        Decimal::new_from_big_rat_with_prec(&fraction, 2, M::Unnecessary),
    ] {
        assert_error(result, ErrorKind::RoundUnnecessary);
    }
    assert_eq!(a.string_with_trailing_zeros(), "1.2300");
    assert_eq!(zero.scale(), isize::MAX as usize);
    assert!(!zero.is_nil());
}

#[test]
fn invalid_modes_float_inputs_and_format_precision_return_errors() {
    for raw in [i32::MIN, -1, 7, i32::MAX] {
        assert_error(M::from_raw(raw), ErrorKind::InvalidArgument);
        assert_eq!(M::from_raw(raw), M::try_from(raw));
    }
    for f in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_error(Decimal::new_from_float64(f), ErrorKind::InvalidFormat);
        assert_error(
            Decimal::new_from_float32(f as f32),
            ErrorKind::InvalidFormat,
        );
    }
    for verb in ['f', 'F'] {
        assert_error(
            Decimal::new(0).format_go(GoFormat {
                verb,
                precision: Some(isize::MAX as usize + 1),
                ..GoFormat::default()
            }),
            ErrorKind::InvalidPrecision,
        );
    }
    assert_error(
        rs_decimal::most_significant_bit(&(-1).into()),
        ErrorKind::InvalidArgument,
    );
    for figures in [0, -1, isize::MIN] {
        assert_error(
            Decimal::default().significant_figures(figures, M::Down),
            ErrorKind::InvalidArgument,
        );
    }
    assert_error(
        Decimal::new(-2).safe_add(&Decimal::new(1)),
        ErrorKind::InvalidArgument,
    );
    assert_error(
        Decimal::new(1).safe_sub(&Decimal::new(2)),
        ErrorKind::InvalidArgument,
    );
}

#[test]
fn explicit_must_methods_keep_their_panic_contracts() {
    let payload = std::panic::catch_unwind(|| Decimal::must_from_string("invalid")).unwrap_err();
    assert!(
        payload
            .downcast_ref::<DecimalError>()
            .unwrap()
            .matches(ErrorKind::InvalidFormat)
    );
    let payload = std::panic::catch_unwind(|| Decimal::new(-1).must_non_negative()).unwrap_err();
    assert_eq!(payload.downcast_ref::<&str>(), Some(&"Negative value"));
}
