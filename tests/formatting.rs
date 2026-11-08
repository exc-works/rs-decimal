use rs_decimal::{Decimal, GoFormat};
#[test]
fn general_rounding_crosses_notation_threshold_once() {
    for (input, p, hash, want) in [
        ("999.9", 3, false, "1e+03"),
        ("999.9", 3, true, "1.00e+03"),
        ("0.00009999", 3, false, "0.0001"),
        ("0.00009999", 3, true, "0.000100"),
        ("999.5", 3, false, "1e+03"),
        ("998.5", 3, false, "998"),
        ("9.5", 1, true, "1.e+01"),
        ("1.25", 2, false, "1.2"),
        ("1.35", 2, false, "1.4"),
    ] {
        let d = Decimal::must_from_string(input);
        assert_eq!(
            d.format_go(GoFormat {
                verb: 'g',
                precision: Some(p),
                alternate: hash,
                ..GoFormat::default()
            })
            .unwrap(),
            want
        );
    }
}
#[test]
fn scientific_extreme_scale_does_not_expand_zeros() {
    for scale in [131072, isize::MAX] {
        for c in [1, -100] {
            let d = Decimal::new_with_prec(c, scale).unwrap();
            let want = if c == 1 {
                format!("1.000000e-{scale}")
            } else {
                format!("-1.000000e-{}", scale - 2)
            };
            assert_eq!(
                d.format_go(GoFormat {
                    verb: 'e',
                    ..GoFormat::default()
                })
                .unwrap(),
                want
            );
        }
    }
}
#[test]
fn append_zero_state_separators_and_padding() {
    let d = Decimal::must_from_string("-1234.500");
    let mut bytes = vec![0xff, 0];
    d.append_text(&mut bytes);
    assert_eq!(&bytes[2..], b"-1234.5");
    let mut bytes = Vec::new();
    d.append_text_with_trailing_zeros(&mut bytes);
    assert_eq!(bytes, b"-1234.500");
    assert_eq!(
        d.format_with_separators('\u{202f}', ','),
        "-1\u{202f}234,500"
    );
    let z = Decimal::new_with_prec(0, 3).unwrap();
    assert_eq!(z.string(), "0");
    assert_eq!(z.string_with_trailing_zeros(), "0.000");
    let nil = Decimal::default();
    assert_eq!(nil.string(), "0");
    assert!(nil.is_nil());
    let d = Decimal::new(12);
    assert_eq!(
        d.format_go(GoFormat {
            verb: 'd',
            width: Some(5),
            precision: Some(3),
            zero: true,
            ..GoFormat::default()
        })
        .unwrap(),
        "  012"
    );
    assert_eq!(
        Decimal::new_with_prec(12355, 3)
            .unwrap()
            .format_go(GoFormat {
                verb: 'f',
                width: Some(10),
                precision: Some(2),
                plus: true,
                zero: true,
                ..GoFormat::default()
            })
            .unwrap(),
        "+000012.36"
    );
    assert_eq!(
        format!("{}", Decimal::new_with_prec(12300, 3).unwrap()),
        "12.3"
    );
}
