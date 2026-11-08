use rs_decimal::{BigInt, Decimal, ErrorKind, RoundingMode};
use std::ops::{Add, Neg, Sub};

fn state(d: &Decimal) -> (BigInt, usize, bool) {
    (d.big_int(), d.scale(), d.is_nil())
}

#[test]
fn primitive_conversions_cover_each_full_integer_range() {
    macro_rules! check {
        ($($integer:ty),*) => { $(
            for input in [<$integer>::MIN, 0, 1, <$integer>::MAX] {
                let d = Decimal::from(input);
                let expected: Decimal = input.to_string().parse().unwrap();
                assert_eq!(state(&d), state(&expected), "{}: {input}", stringify!($integer));
            }
        )* };
    }
    check!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    );
    let text = "-12345678901234567890123456789012345678901234567890";
    let coefficient: BigInt = text.parse().unwrap();
    let d = Decimal::from(coefficient);
    assert_eq!(d.to_string(), text);
    assert_eq!(d.scale(), 0);
    assert!(!d.is_nil());
    assert!(!Decimal::from(BigInt::from(0)).is_nil());
}

#[test]
fn borrowed_operator_traits_keep_legacy_calls_and_inputs_usable() {
    let values = [
        Decimal::default(),
        Decimal::new_with_prec(0, 4),
        Decimal::new_with_prec(-123, 2),
        Decimal::new_with_prec(123, 3),
        Decimal::from(i128::MIN),
        Decimal::from(u128::MAX),
    ];
    for a in &values {
        for b in &values {
            let a = a.clone();
            let b = b.clone();
            let before = (state(&a), state(&b));
            // These legacy dot calls must still borrow even with traits imported.
            let sum = a.add(&b);
            let difference = a.sub(&b);
            let negated = a.neg();
            assert_eq!(state(&(&a + &b)), state(&sum));
            assert_eq!(state(&Add::add(&a, &b)), state(&sum));
            assert_eq!(state(&(&a - &b)), state(&difference));
            assert_eq!(state(&Sub::sub(&a, &b)), state(&difference));
            assert_eq!(state(&(-&a)), state(&negated));
            assert_eq!(state(&Neg::neg(&a)), state(&negated));
            assert_eq!((state(&a), state(&b)), before);
        }
    }
}

#[test]
fn fallible_modes_preserve_legacy_mapping_and_panic() {
    for (raw, expected) in [
        RoundingMode::Down,
        RoundingMode::Up,
        RoundingMode::Ceiling,
        RoundingMode::HalfUp,
        RoundingMode::HalfDown,
        RoundingMode::HalfEven,
        RoundingMode::Unnecessary,
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(RoundingMode::try_from(raw as i32).unwrap(), expected);
        assert_eq!(RoundingMode::from_raw(raw as i32), expected);
    }
    for raw in [i32::MIN, -1, 7, i32::MAX] {
        assert!(
            RoundingMode::try_from(raw)
                .unwrap_err()
                .matches(ErrorKind::InvalidArgument)
        );
        let payload = std::panic::catch_unwind(|| RoundingMode::from_raw(raw)).unwrap_err();
        let message = payload
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| payload.downcast_ref::<String>().map(String::as_str));
        assert_eq!(message, Some("invalid rounding mode"));
    }
}
