use rs_decimal::{BigInt, Decimal, DecimalError, ErrorKind, NullDecimal};

#[test]
fn parsing_preserves_representation_and_normalizes_zero() {
    for (s, c, p) in [
        (" +001.2300 ", "12300", 4),
        ("-1.2e3", "-1200", 0),
        ("1e-3", "1", 3),
        ("-0.000e2", "0", 0),
        ("1.00e-2", "100", 4),
    ] {
        let d = Decimal::new_from_string(s).unwrap();
        assert_eq!(d.big_int().to_string(), c);
        assert_eq!(d.precision(), p);
        assert!(!d.is_nil());
    }
    for s in [
        "",
        " ",
        ".1",
        "1.",
        "1.2.3",
        "1e",
        "1e--2",
        "1_000",
        "--1",
        "1e2147483648",
        "0e131073",
    ] {
        assert!(
            Decimal::new_from_string(s)
                .unwrap_err()
                .matches(ErrorKind::InvalidFormat),
            "{s}"
        );
    }
    assert_eq!(
        Decimal::new_from_string("1e-131072").unwrap().precision(),
        131072
    );
}

#[test]
fn zero_state_equality_and_equal_extrema() {
    let nil = Decimal::default();
    let scaled = Decimal::new_with_prec(0, 30).unwrap();
    assert_eq!(nil, scaled);
    assert!(nil.is_nil());
    assert!(!scaled.is_nil());
    assert!(Decimal::max(&nil, &scaled).is_nil());
    assert_eq!(Decimal::min(&scaled, &nil).precision(), 30);
    assert!(nil.clone().is_nil());
    assert!(!nil.abs().is_nil());
    assert!(!nil.neg().is_nil());
    let valid = NullDecimal::new(nil);
    assert!(valid.valid);
    assert!(valid.decimal.is_nil());
}

#[test]
fn comparison_handles_extreme_scale_and_signs_without_expansion() {
    for sign in [-1, 1] {
        let tiny = Decimal::new_with_prec(sign, isize::MAX).unwrap();
        let ordinary = Decimal::new(sign);
        assert_eq!(tiny.cmp(&ordinary), -sign as i32);
        assert_eq!(ordinary.cmp(&tiny), sign as i32);
    }
    assert_eq!(
        Decimal::must_from_string("1.0"),
        Decimal::must_from_string("1.00")
    );
    assert_ne!(Decimal::new(12), Decimal::new_with_prec(12, 1).unwrap());
}

#[test]
fn constructors_own_coefficients_and_keep_scale() {
    let mut c = BigInt::from(123);
    let d = Decimal::new_from_big_int_with_prec(&c, 2).unwrap();
    c += 1;
    let mut copy = d.big_int();
    copy += 1;
    assert_eq!(d.string(), "1.23");
    assert_eq!(c, copy);
    assert_eq!(
        Decimal::new_with_append_prec(12, 3).unwrap().big_int(),
        BigInt::from(12000)
    );
    assert_eq!(Decimal::new_with_append_prec(12, 3).unwrap().precision(), 3);
    let p = Decimal::new_with_prec(1, -1).unwrap_err();
    assert!(p.matches(ErrorKind::InvalidPrecision));
}

#[test]
fn errors_keep_all_categories() {
    let e =
        DecimalError::new(ErrorKind::InvalidFormat, "bad number").with_kind(ErrorKind::Unmarshal);
    assert!(e.matches(ErrorKind::InvalidFormat));
    assert!(e.matches(ErrorKind::Unmarshal));
    assert_eq!(e.clone().with_kind(ErrorKind::Unmarshal).kinds().len(), 2);
}

#[test]
fn comparison_matches_independent_rational_order() {
    use rs_decimal::BigRational;
    for a in -30..=30 {
        for b in -30..=30 {
            let sa = (a + 30) % 6;
            let sb = (b + 30) % 7;
            let x = Decimal::new_with_prec(a, sa as isize).unwrap();
            let y = Decimal::new_with_prec(b, sb as isize).unwrap();
            let ra = BigRational::new(BigInt::from(a), BigInt::from(10).pow(sa as u32));
            let rb = BigRational::new(BigInt::from(b), BigInt::from(10).pow(sb as u32));
            assert_eq!(x.cmp(&y), ra.cmp(&rb) as i32);
        }
    }
}

#[test]
fn native_limits_and_constant_mappings() {
    assert_eq!(Decimal::new(i64::MIN).big_int(), BigInt::from(i64::MIN));
    assert_eq!(
        Decimal::new_from_uint64(u64::MAX, 2).unwrap().big_int(),
        BigInt::from(u64::MAX)
    );
    assert_eq!(rs_decimal::zero(), Decimal::new(0));
    assert_eq!(rs_decimal::one(), Decimal::new(1));
    assert_eq!(rs_decimal::ten(), Decimal::new(10));
    assert_eq!(rs_decimal::hundred(), Decimal::new(100));
    let a = Decimal::new_with_prec(10, 1).unwrap();
    let b = Decimal::new(1);
    assert_eq!(rs_decimal::max(&a, &b).precision(), 1);
    assert_eq!(rs_decimal::min(&a, &b).precision(), 1);
    assert_eq!(Ord::cmp(&a, &b), std::cmp::Ordering::Equal);
}

#[test]
fn parser_small_coefficient_boundary_preserves_digits_sign_and_scale() {
    for digits in [
        "99999999999999999",
        "999999999999999999",
        "9999999999999999999",
    ] {
        for sign in ["", "+", "-"] {
            for point in [0, 1, 8] {
                let split = digits.len() - point;
                let text = if point == 0 {
                    format!("{sign}{digits}")
                } else {
                    format!("{sign}{}.{}", &digits[..split], &digits[split..])
                };
                let parsed: rs_decimal::Decimal = text.parse().unwrap();
                let coefficient: rs_decimal::BigInt = format!("{sign}{digits}").parse().unwrap();
                assert_eq!(parsed.big_int(), coefficient);
                assert_eq!(parsed.precision(), point as isize);
                assert!(!parsed.is_nil());
            }
        }
    }
    for text in ["-000000000000000000.00", "+0000000000000000000e-5"] {
        let parsed: rs_decimal::Decimal = text.parse().unwrap();
        assert_eq!(parsed.big_int(), rs_decimal::BigInt::from(0));
        assert_eq!(parsed.precision(), 0);
        assert!(!parsed.is_nil());
    }
}
