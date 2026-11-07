use rs_decimal::{Decimal, ErrorKind};

#[test]
fn certified_operations_reject_excessive_precision_before_work() {
    let two = Decimal::new(2);
    for result in [
        two.ln_with_prec(131073),
        two.exp_with_prec(131073),
        two.sqrt_with_prec(131073),
        two.approx_root_with_prec(131073, 1),
    ] {
        assert!(result.unwrap_err().matches(ErrorKind::NoConvergence));
    }
    assert!("1e131073".parse::<Decimal>().is_err());
    assert!("1e-131073".parse::<Decimal>().is_err());
}

#[test]
fn rejected_binary_scale_preserves_representation() {
    let mut value = Decimal::must_from_string("12.3400");
    let before = (value.big_int(), value.precision(), value.is_nil());
    for scale in [131073u32, u32::MAX] {
        let mut wire = scale.to_be_bytes().to_vec();
        wire.extend_from_slice(&[2, 1]);
        assert!(value.unmarshal_binary(&wire).is_err());
        assert_eq!((value.big_int(), value.precision(), value.is_nil()), before);
    }
}
