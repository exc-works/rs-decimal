use rs_decimal::{Decimal, ErrorKind, NullDecimal, ScanValue};
#[test]
fn precise_scan_and_values() {
    let mut d = Decimal::default();
    assert_eq!(d.value(), "0");
    assert!(d.is_nil());
    d.scan(ScanValue::Text("\"9007199254740993.000100\""))
        .unwrap();
    assert_eq!(d.string_with_trailing_zeros(), "9007199254740993.000100");
    assert_eq!(d.value(), "9007199254740993.0001");
    d.scan(ScanValue::Float32(0.1)).unwrap();
    assert_eq!(d.string(), "0.1");
    d.scan(ScanValue::Int64(i64::MIN)).unwrap();
    assert_eq!(d.string(), "-9223372036854775808");
    d.scan(ScanValue::Null).unwrap();
    assert!(d.is_nil());
}
#[test]
fn nullable_and_atomic_errors() {
    let mut n = NullDecimal::new(Decimal::new_with_prec(1230, 2).unwrap());
    for value in [
        ScanValue::Text(""),
        ScanValue::Text("\"\""),
        ScanValue::Bytes(&[255]),
        ScanValue::Text("\"1\\u0032\""),
        ScanValue::Float64(f64::INFINITY),
        ScanValue::Unsupported,
    ] {
        assert!(n.scan(value).unwrap_err().matches(ErrorKind::Unmarshal));
        assert!(n.valid);
        assert_eq!(n.decimal.string_with_trailing_zeros(), "12.30");
    }
    n.scan(ScanValue::Null).unwrap();
    assert!(!n.valid && n.decimal.is_nil());
    assert_eq!(n.value(), None);
    n = NullDecimal::new(Decimal::default());
    assert_eq!(n.value(), Some("0".into()));
    assert!(n.decimal.is_nil());
}
