use rs_decimal::{BigInt, BigRational, Decimal, DecimalError, ErrorKind, RoundingMode as M};

#[test]
fn exact_rational_factorization_and_rounding() {
    for twos in [0usize, 1, 7, 1000, 10000] {
        for fives in [0, 1, 9, 1000] {
            let den = (BigInt::from(1) << twos) * BigInt::from(5).pow(fives);
            for sign in [-7, 7] {
                let r = BigRational::new(sign.into(), den.clone());
                let d = Decimal::new_from_big_rat(&r).unwrap();
                assert_eq!(d.big_rat(), r);
                assert_eq!(d.precision(), twos.max(fives as usize) as isize);
            }
        }
    }
    assert!(
        Decimal::new_from_big_rat(&BigRational::new(1.into(), 3.into()))
            .unwrap_err()
            .matches(ErrorKind::InvalidFormat)
    );
    assert_eq!(
        Decimal::new_from_big_rat_with_prec(&BigRational::new(1.into(), 3.into()), 3, M::HalfEven)
            .unwrap()
            .string(),
        "0.333"
    );
}
#[test]
fn shortest_float_inputs_and_nonfinite_rejection() {
    assert_eq!(Decimal::new_from_float64(0.1).string(), "0.1");
    assert_eq!(Decimal::new_from_float32(0.1).precision(), 1);
    assert_eq!(Decimal::new_from_float64(1.0).precision(), 0);
    assert_eq!(Decimal::new_from_float64(-0.0).sign(), 0);
    for f in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let e = std::panic::catch_unwind(|| Decimal::new_from_float64(f)).unwrap_err();
        assert!(
            e.downcast_ref::<DecimalError>()
                .unwrap()
                .matches(ErrorKind::InvalidFormat)
        );
    }
}
#[test]
fn float_boundaries_and_exact_flags() {
    assert_eq!(Decimal::must_from_string("0.5").float64(), (0.5, true));
    assert_eq!(Decimal::must_from_string("0.1").float64(), (0.1, false));
    for bits in [1, 2, 0x007fffff, 0x00800000, 0x3f800000, 0x7f7fffff] {
        let f = f32::from_bits(bits);
        let r = BigRational::from_float(f).unwrap();
        let d = Decimal::new_from_big_rat(&r).unwrap();
        assert_eq!(d.float32().0.to_bits(), bits);
        assert!(d.float32().1);
    }
    for bits in [
        1,
        2,
        0x000fffffffffffff,
        0x0010000000000000,
        0x3ff0000000000000,
        0x7fefffffffffffff,
    ] {
        let f = f64::from_bits(bits);
        let r = BigRational::from_float(f).unwrap();
        let d = Decimal::new_from_big_rat(&r).unwrap();
        assert_eq!(d.float64().0.to_bits(), bits);
        assert!(d.float64().1);
    }
    assert_eq!(
        Decimal::must_from_string("-1e-400").float64().0.to_bits(),
        1 << 63
    );
    assert_eq!(
        Decimal::must_from_string("1e400").float64(),
        (f64::INFINITY, false)
    );
    // Slightly above an f32 midpoint; converting through f64 would erase this offset.
    let n = (BigInt::from(1) << 80) + (BigInt::from(1) << 56) + 1;
    let d = Decimal::new_from_big_rat(&BigRational::new(n, BigInt::from(1) << 80)).unwrap();
    assert_eq!(d.float32().0.to_bits(), 0x3f800001);
}
#[test]
fn native_integer_exactness_and_bits() {
    for s in ["1.2", "9223372036854775808", "-9223372036854775809"] {
        assert_eq!(Decimal::must_from_string(s).int64(), (0, false));
    }
    assert_eq!(Decimal::new(i64::MIN).int64(), (i64::MIN, true));
    assert_eq!(
        Decimal::new_from_uint64(u64::MAX, 0).uint64(),
        (u64::MAX, true)
    );
    assert_eq!(Decimal::new(-1).uint64(), (0, false));
    assert_eq!(Decimal::new_with_prec(0, isize::MAX).int64(), (0, true));
    assert_eq!(Decimal::new(8).bit_len(), 4);
    assert_eq!(rs_decimal::most_significant_bit(&8.into()), 3);
    assert_eq!(rs_decimal::most_significant_bit(&0.into()), 0);
    assert!(std::panic::catch_unwind(|| rs_decimal::most_significant_bit(&(-1).into())).is_err());
}
