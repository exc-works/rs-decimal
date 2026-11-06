use rs_decimal::{Decimal, ErrorKind, NullDecimal};
#[test]
fn text_and_param_transitions_are_atomic() {
    for valid in [false, true] {
        for initial in [
            Decimal::default(),
            Decimal::new_with_prec(0, 3),
            Decimal::new_with_prec(1230, 2),
        ] {
            let n = NullDecimal {
                decimal: initial.clone(),
                valid,
            };
            assert_eq!(
                n.marshal_text(),
                if valid {
                    initial.marshal_text()
                } else {
                    vec![]
                }
            );
            for bad in [b" ".as_slice(), b"x", b"\xff", b".1"] {
                let mut d = initial.clone();
                let err = d.unmarshal_text(bad).unwrap_err();
                assert!(err.matches(ErrorKind::Unmarshal) && err.matches(ErrorKind::InvalidFormat));
                assert_eq!(d.big_int(), initial.big_int());
                assert_eq!(d.precision(), initial.precision());
                assert_eq!(d.is_nil(), initial.is_nil());
                let mut x = n.clone();
                assert!(x.unmarshal_text(bad).is_err());
                assert_eq!(x.valid, n.valid);
                assert_eq!(x.decimal.precision(), initial.precision());
                assert_eq!(x.decimal.is_nil(), initial.is_nil());
            }
            let mut x = n.clone();
            x.unmarshal_param("").unwrap();
            assert!(!x.valid && x.decimal.is_nil());
            let mut d = initial.clone();
            assert!(d.unmarshal_param("").is_err());
            let mut x = n.clone();
            x.unmarshal_param(" 1.2300 ").unwrap();
            assert!(x.valid);
            assert_eq!(x.decimal.string_with_trailing_zeros(), "1.2300");
        }
    }
}
#[test]
fn valid_nil_encodes_zero_without_initializing() {
    let n = NullDecimal::new(Decimal::default());
    assert_eq!(n.string(), "0");
    assert_eq!(n.marshal_text(), b"0");
    assert!(n.decimal.is_nil());
    assert_eq!(NullDecimal::default().string(), "null");
}
