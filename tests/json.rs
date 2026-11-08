#![cfg(feature = "json")]
use rs_decimal::{Decimal, ErrorKind, NullDecimal};
#[test]
fn exact_tokens_null_and_failed_updates() {
    let mut d = Decimal::new_with_prec(1230, 2).unwrap();
    d.unmarshal_json(b"null").unwrap();
    assert_eq!(d.string_with_trailing_zeros(), "12.30");
    for token in [
        "9007199254740993",
        "123456789012345678901234567890.123456789",
    ] {
        d.unmarshal_json(token.as_bytes()).unwrap();
        assert_eq!(d.string(), token);
    }
    let original = d.clone();
    for token in [
        b"\x0bnull".as_slice(),
        b"null\x0c",
        b"01",
        b"{}",
        b"[]",
        b"true",
        b"NaN",
        b"\"\xff\"",
        br#""\ud800""#,
    ] {
        let e = d.unmarshal_json(token).unwrap_err();
        assert!(e.matches(ErrorKind::Unmarshal));
        assert_eq!(d, original);
        assert_eq!(d.precision(), original.precision());
    }
    assert!(
        d.unmarshal_json(b"\"\xff\"")
            .unwrap_err()
            .matches(ErrorKind::InvalidFormat)
    );
    let mut n = NullDecimal::new(Decimal::default());
    assert_eq!(n.marshal_json(), b"\"0\"");
    assert!(n.decimal.is_nil());
    assert_eq!(Decimal::default().marshal_json(), b"null");
    n.unmarshal_json(b"null").unwrap();
    assert!(!n.valid && n.decimal.is_nil());
    let mut x = Decimal::default();
    assert!(x.unmarshal_json(b"\"\"").is_err());
    assert!(x.is_nil());
}
#[cfg(feature = "serde")]
#[test]
fn serde_exact_strings_integers_and_missing_fields() {
    let d = Decimal::must_from_string("12345678901234567890.12300");
    let encoded = serde_json::to_string(&d).unwrap();
    assert_eq!(encoded, "\"12345678901234567890.123\"");
    let decoded: Decimal = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded, d);
    let decoded: Decimal = serde_json::from_str("9007199254740993").unwrap();
    assert_eq!(decoded.string(), "9007199254740993");
    // Generic Serde cannot recover a token after a deserializer rounds it to f64.
    assert!(serde_json::from_str::<Decimal>("1.234567890123456789").is_err());
    assert!(serde_json::from_str::<Decimal>(r#"{"$serde_json::private::Number":"1.23"}"#).is_err());
    assert!(serde_json::from_str::<Decimal>("null").unwrap().is_nil());
    let invalid: NullDecimal = serde_json::from_str("null").unwrap();
    assert!(!invalid.valid && invalid.decimal.is_nil());
    assert_eq!(
        serde_json::to_string(&NullDecimal::new(Decimal::default())).unwrap(),
        "\"0\""
    );
    let missing: std::collections::BTreeMap<String, Decimal> = serde_json::from_str("{}").unwrap();
    assert!(missing.is_empty());
}
