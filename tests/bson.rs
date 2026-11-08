#![cfg(feature = "bson")]
use rs_decimal::{Decimal, ErrorKind, NullDecimal};
#[test]
fn exact_wire_and_decimal128() {
    let d = Decimal::must_from_string("1.2300");
    assert_eq!(
        d.marshal_bson_value().unwrap(),
        (2, vec![5, 0, 0, 0, b'1', b'.', b'2', b'3', 0])
    );
    let mut decoded = Decimal::default();
    let mut data = 12300u64.to_le_bytes().to_vec();
    data.extend_from_slice(&((6176u64 - 4) << 49).to_le_bytes());
    decoded.unmarshal_bson_value(19, &data).unwrap();
    assert_eq!(decoded.string_with_trailing_zeros(), "1.2300");
    decoded
        .unmarshal_bson_value(18, &i64::MAX.to_le_bytes())
        .unwrap();
    assert_eq!(decoded.string(), "9223372036854775807");
    decoded
        .unmarshal_bson_value(2, &[3, 0, 0, 0, b'1', b'2', b'x', b'e'])
        .unwrap();
    assert_eq!(decoded, Decimal::new(12));
}
#[test]
fn failure_and_nullable_states() {
    let mut n = NullDecimal::new(Decimal::new_with_prec(1230, 2).unwrap());
    for (kind, data) in [
        (19, vec![0; 15]),
        (2, vec![255; 4]),
        (1, f64::NAN.to_le_bytes().to_vec()),
        (8, vec![1]),
    ] {
        assert!(
            n.unmarshal_bson_value(kind, &data)
                .unwrap_err()
                .matches(ErrorKind::Unmarshal)
        );
        assert!(n.valid);
        assert_eq!(n.decimal.string_with_trailing_zeros(), "12.30");
    }
    let mut infinity = vec![0; 8];
    infinity.extend_from_slice(&0x7800000000000000u64.to_le_bytes());
    assert!(
        n.unmarshal_bson_value(19, &infinity)
            .unwrap_err()
            .matches(ErrorKind::InvalidFormat)
    );
    n.unmarshal_bson_value(10, b"ignored").unwrap();
    assert!(!n.valid && n.decimal.is_nil());
    assert_eq!(n.marshal_bson_value().unwrap(), (10, vec![]));
    n = NullDecimal::new(Decimal::default());
    assert_eq!(
        n.marshal_bson_value().unwrap(),
        (2, vec![2, 0, 0, 0, b'0', 0])
    );
    assert!(n.decimal.is_nil());
}
