use rs_decimal::{BigInt, Decimal, ErrorKind};
#[test]
fn defaults_match_explicit_scales_and_preserve_inputs() {
    for s in [0, 20, 30] {
        let d = Decimal::new_with_append_prec(3, s).unwrap();
        let before = d.clone();
        let default = [
            d.ln().unwrap(),
            d.log2().unwrap(),
            d.log10().unwrap(),
            d.exp().unwrap(),
        ];
        let explicit = [
            d.ln_with_prec(s.max(20)).unwrap(),
            d.log2_with_prec(s.max(20)).unwrap(),
            d.log10_with_prec(s.max(20)).unwrap(),
            d.exp_with_prec(s.max(20)).unwrap(),
        ];
        for (a, b) in default.iter().zip(explicit.iter()) {
            assert_eq!(a.big_int(), b.big_int());
            assert_eq!(a.precision(), s.max(20));
            assert!(!a.is_nil());
        }
        for p in [0, 2, 40] {
            for a in [
                d.ln_with_prec(p),
                d.log2_with_prec(p),
                d.log10_with_prec(p),
                d.exp_with_prec(p),
            ] {
                assert_eq!(a.unwrap().precision(), p);
            }
        }
        assert_eq!(d.big_int(), before.big_int());
        assert_eq!(d.precision(), before.precision());
    }
}
#[test]
fn exact_identities_domains_and_budget_order() {
    assert_eq!(Decimal::new(8).log2().unwrap().string(), "3");
    assert_eq!(
        Decimal::must_from_string("0.001").log10().unwrap().string(),
        "-3"
    );
    assert_eq!(
        Decimal::new(1).ln_with_prec(131072).unwrap().precision(),
        131072
    );
    assert_eq!(
        Decimal::new(0).exp_with_prec(131072).unwrap().precision(),
        131072
    );
    assert_eq!(
        Decimal::new(-1000000)
            .exp_with_prec(1000)
            .unwrap()
            .big_int(),
        BigInt::from(0)
    );
    for d in [Decimal::default(), Decimal::new(-1)] {
        for e in [d.ln(), d.log10(), d.log2_with_prec(20)] {
            assert!(e.unwrap_err().matches(ErrorKind::InvalidLog));
        }
        let e = d.log2().unwrap_err();
        assert!(e.matches(ErrorKind::InvalidLog));
    }
    for d in [
        Decimal::new_with_prec(0, 131073).unwrap(),
        Decimal::new_with_prec(1, 131073).unwrap(),
    ] {
        assert!(d.exp().unwrap_err().matches(ErrorKind::NoConvergence));
    }
    assert!(
        Decimal::new(1000000)
            .exp()
            .unwrap_err()
            .matches(ErrorKind::NoConvergence)
    );
    assert!(
        Decimal::new(2)
            .ln_with_prec(131072)
            .unwrap_err()
            .matches(ErrorKind::NoConvergence)
    );
    for f in [
        Decimal::ln_with_prec,
        Decimal::log2_with_prec,
        Decimal::log10_with_prec,
        Decimal::exp_with_prec,
    ] {
        let e = f(&Decimal::new(-1), -1).unwrap_err();
        assert!(e.matches(ErrorKind::InvalidPrecision));
    }
}
