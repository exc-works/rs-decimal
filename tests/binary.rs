use rs_decimal::{Decimal, ErrorKind, PRECISION_FIXED_SIZE};
#[test]
fn go_golden_bytes_and_noncanonical_inputs() {
    assert_eq!(PRECISION_FIXED_SIZE, 4);
    for (text, bytes) in [
        ("0", vec![0, 0, 0, 0, 2]),
        ("1.23", vec![0, 0, 0, 2, 2, 123]),
        ("-1.23", vec![0, 0, 0, 2, 3, 123]),
        ("1.2300", vec![0, 0, 0, 2, 2, 123]),
    ] {
        let d = Decimal::must_from_string(text);
        assert_eq!(d.marshal_binary().unwrap(), bytes);
        let mut decoded = Decimal::default();
        decoded.unmarshal_binary(&bytes).unwrap();
        assert_eq!(decoded, d);
        assert!(!decoded.is_nil());
    }
    for payload in [vec![], vec![2], vec![3], vec![2, 0, 0], vec![3, 0]] {
        let mut bytes = vec![0, 0, 0, 3];
        bytes.extend(payload);
        let mut d = Decimal::new(123);
        d.unmarshal_binary(&bytes).unwrap();
        assert!(d.is_zero());
        assert_eq!(d.precision(), 3);
    }
    let mut nil = Decimal::default();
    let bytes = nil.marshal_binary().unwrap();
    assert!(bytes.is_empty());
    nil.unmarshal_binary(&bytes).unwrap();
    assert!(nil.is_zero() && !nil.is_nil());
}
#[test]
fn failure_atomicity_and_fixed_width_precision_limits() {
    let mut d = Decimal::new_with_prec(1230, 2).unwrap();
    for bytes in [
        vec![0],
        vec![0, 0, 0],
        vec![0, 0, 0, 1, 0],
        vec![0, 0, 0, 1, 4],
        vec![0, 2, 0, 1, 2],
        vec![255, 255, 255, 255, 2],
    ] {
        assert!(
            d.unmarshal_binary(&bytes)
                .unwrap_err()
                .matches(ErrorKind::Unmarshal)
        );
        assert_eq!(d.string_with_trailing_zeros(), "12.30");
    }
    d.unmarshal_binary(&[0, 2, 0, 0, 2, 1]).unwrap();
    assert_eq!(d.precision(), 131072);
    assert!(
        Decimal::new_with_prec(1, 131073)
            .unwrap()
            .marshal_binary()
            .unwrap_err()
            .matches(ErrorKind::InvalidPrecision)
    );
    assert!(
        Decimal::new_with_prec(10, 131073)
            .unwrap()
            .marshal_binary()
            .is_ok()
    );
    assert_eq!(
        Decimal::new_with_prec(0, isize::MAX)
            .unwrap()
            .marshal_binary()
            .unwrap(),
        vec![0, 0, 0, 0, 2]
    );
}
