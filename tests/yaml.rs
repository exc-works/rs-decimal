#![cfg(feature = "yaml")]
use rs_decimal::{Decimal, ErrorKind, NullDecimal};
// Cases copied from the immutable upstream yaml_scalars.json manifest.
#[test]
fn fixed_scalar_syntax_and_exact_digits() {
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("012").unwrap();
        assert_eq!(d, Decimal::must_from_string("10"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("-012").unwrap();
        assert_eq!(d, Decimal::must_from_string("-10"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("+012").unwrap();
        assert_eq!(d, Decimal::must_from_string("10"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("0o12").unwrap();
        assert_eq!(d, Decimal::must_from_string("10"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("0x10").unwrap();
        assert_eq!(d, Decimal::must_from_string("16"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("-0x10").unwrap();
        assert_eq!(d, Decimal::must_from_string("-16"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("0b1010").unwrap();
        assert_eq!(d, Decimal::must_from_string("10"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("1_000").unwrap();
        assert_eq!(d, Decimal::must_from_string("1000"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("0x1_e2").unwrap();
        assert_eq!(d, Decimal::must_from_string("482"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml(".5").unwrap();
        assert_eq!(d, Decimal::must_from_string("0.5"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("-.5").unwrap();
        assert_eq!(d, Decimal::must_from_string("-0.5"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("1.").unwrap();
        assert_eq!(d, Decimal::must_from_string("1"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("1.e2").unwrap();
        assert_eq!(d, Decimal::must_from_string("100"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("1_2.3_4e+2").unwrap();
        assert_eq!(d, Decimal::must_from_string("1234"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("012.5").unwrap();
        assert_eq!(d, Decimal::must_from_string("12.5"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("0128").unwrap();
        assert_eq!(d, Decimal::must_from_string("128"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("\"012\"").unwrap();
        assert_eq!(d, Decimal::must_from_string("12"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("'012'").unwrap();
        assert_eq!(d, Decimal::must_from_string("12"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("!!str 012").unwrap();
        assert_eq!(d, Decimal::must_from_string("12"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("!!int \"012\"").unwrap();
        assert_eq!(d, Decimal::must_from_string("10"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("!!float 012").unwrap();
        assert_eq!(d, Decimal::must_from_string("10"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("!!float 0x10").unwrap();
        assert_eq!(d, Decimal::must_from_string("16"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("!!float 0b1010").unwrap();
        assert_eq!(d, Decimal::must_from_string("10"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("0123456701234567012345670123456701234567")
            .unwrap();
        assert_eq!(
            d,
            Decimal::must_from_string("123456701234567012345670123456701234567")
        );
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("1.23456789012345678901234567890").unwrap();
        assert_eq!(
            d,
            Decimal::must_from_string("1.23456789012345678901234567890")
        );
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("9007199254740993.0").unwrap();
        assert_eq!(d, Decimal::must_from_string("9007199254740993"));
    }
    {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml("1_234.56789012345678901234567890")
            .unwrap();
        assert_eq!(
            d,
            Decimal::must_from_string("1234.56789012345678901234567890")
        );
    }
    {
        let mut d = Decimal::new(9);
        assert!(d.unmarshal_yaml("\"0x10\"").is_err());
        assert_eq!(d, Decimal::new(9));
    }
    {
        let mut d = Decimal::new(9);
        assert!(d.unmarshal_yaml("\"1_000\"").is_err());
        assert_eq!(d, Decimal::new(9));
    }
    {
        let mut d = Decimal::new(9);
        assert!(d.unmarshal_yaml("\".5\"").is_err());
        assert_eq!(d, Decimal::new(9));
    }
    {
        let mut d = Decimal::new(9);
        assert!(d.unmarshal_yaml("\"1.\"").is_err());
        assert_eq!(d, Decimal::new(9));
    }
    {
        let mut d = Decimal::new(9);
        assert!(d.unmarshal_yaml("!!str 0x10").is_err());
        assert_eq!(d, Decimal::new(9));
    }
    {
        let mut d = Decimal::new(9);
        assert!(d.unmarshal_yaml("true").is_err());
        assert_eq!(d, Decimal::new(9));
    }
    {
        let mut d = Decimal::new(9);
        assert!(d.unmarshal_yaml(".nan").is_err());
        assert_eq!(d, Decimal::new(9));
    }
    {
        let mut d = Decimal::new(9);
        assert!(d.unmarshal_yaml(".inf").is_err());
        assert_eq!(d, Decimal::new(9));
    }
    {
        let mut d = Decimal::new(9);
        assert!(d.unmarshal_yaml("[1, 2]").is_err());
        assert_eq!(d, Decimal::new(9));
    }
}
#[test]
fn null_semantics_and_parser_boundaries() {
    let mut d = Decimal::new_with_prec(1230, 2).unwrap();
    d.unmarshal_yaml("null").unwrap();
    assert_eq!(d.string_with_trailing_zeros(), "12.30");
    let mut n = NullDecimal::new(d);
    n.unmarshal_yaml("~").unwrap();
    assert!(!n.valid && n.decimal.is_nil());
    n.unmarshal_yaml("&anchor 1.2300 # comment").unwrap();
    assert_eq!(n.decimal.string_with_trailing_zeros(), "1.2300");
    assert!(
        n.unmarshal_yaml("[1,2]")
            .unwrap_err()
            .matches(ErrorKind::Unmarshal)
    );
    assert!(n.valid);
    assert!(
        n.unmarshal_yaml("|\n  1.20\n")
            .unwrap_err()
            .matches(ErrorKind::InvalidFormat)
    );
    n.unmarshal_yaml("---\n1\n---\n2").unwrap();
    assert_eq!(n.decimal, Decimal::new(1));
    assert_eq!(
        NullDecimal::new(Decimal::default()).marshal_yaml(),
        Some("0".into())
    );
    assert_eq!(NullDecimal::default().marshal_yaml(), None);
}

#[test]
fn custom_tags_and_later_document_syntax() {
    for text in [
        "!null 123",
        "!bool 123",
        "!int 012",
        "!str 012",
        "!<tag:example.com,2026:null> 123",
    ] {
        let mut d = Decimal::new(9);
        d.unmarshal_yaml(text).unwrap();
        assert_eq!(
            d,
            Decimal::new(if text.ends_with("012") { 12 } else { 123 })
        );
    }
    for text in ["---\n1\n---\n[2", "---\n1\n---\n{a:"] {
        let mut n = NullDecimal::new(Decimal::new_with_prec(1230, 2).unwrap());
        assert!(
            n.unmarshal_yaml(text)
                .unwrap_err()
                .matches(ErrorKind::Unmarshal)
        );
        assert!(n.valid);
        assert_eq!(n.decimal.string_with_trailing_zeros(), "12.30");
    }
}
