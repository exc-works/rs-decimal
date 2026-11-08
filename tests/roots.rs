use rs_decimal::{BigInt, Decimal, ErrorKind};
#[test]
fn defaults_and_explicit_scales() {
    for s in [0, 20, 30] {
        let d = Decimal::new_from_big_int_with_prec(
            &(BigInt::from(2) * BigInt::from(10).pow(s as u32)),
            s,
        )
        .unwrap();
        assert_eq!(d.sqrt().unwrap().precision(), s.max(20));
        assert_eq!(d.approx_root(3).unwrap().precision(), s.max(20));
        assert_eq!(
            d.sqrt().unwrap().big_int(),
            d.sqrt_with_prec(s.max(20)).unwrap().big_int()
        );
        for p in [0, 2, 40] {
            assert_eq!(d.approx_root_with_prec(3, p).unwrap().precision(), p);
        }
    }
}
#[test]
fn midpoint_ties_and_negative_roots() {
    for (s, n, p, c) in [
        ("2.25", 2, 0, 2),
        ("6.25", 2, 0, 2),
        ("-3.375", 3, 0, -2),
        ("-15.625", 3, 0, -2),
    ] {
        assert_eq!(
            Decimal::must_from_string(s)
                .approx_root_with_prec(n, p)
                .unwrap()
                .big_int(),
            BigInt::from(c)
        );
    }
    assert_eq!(
        Decimal::new(2)
            .approx_root_with_prec(500, 50)
            .unwrap()
            .string(),
        "1.00138725571133452908322477441877746756295469052831"
    );
}
#[test]
fn domain_budgets_fast_paths_and_error_order() {
    assert!(
        Decimal::new(-1)
            .sqrt()
            .unwrap_err()
            .matches(ErrorKind::NegativeRoot)
    );
    assert!(
        Decimal::new(2)
            .approx_root(0)
            .unwrap_err()
            .matches(ErrorKind::InvalidRoot)
    );
    assert!(
        Decimal::new(2)
            .approx_root_with_prec(500, 1000)
            .unwrap_err()
            .matches(ErrorKind::NoConvergence)
    );
    assert_eq!(
        Decimal::default()
            .sqrt_with_prec(131073)
            .unwrap()
            .precision(),
        131073
    );
    assert_eq!(
        Decimal::new(1)
            .approx_root_with_prec(i64::MAX, 1)
            .unwrap()
            .big_int(),
        BigInt::from(10)
    );
    let err = Decimal::new(-1).approx_root_with_prec(0, -1).unwrap_err();
    assert!(err.matches(ErrorKind::InvalidPrecision));
}
