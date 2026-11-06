#![cfg(feature = "xml")]
use rs_decimal::{Decimal, ErrorKind, NullDecimal};

#[test]
fn content_attributes_and_exact_digits() {
    let mut d = Decimal::default();
    for source in [
        "<v> 1.2300 </v>",
        "<v><![CDATA[1.2300]]></v>",
        "<v>&#49;.2300</v>",
        "<v>1.<child>ignored</child>2300</v>",
    ] {
        d.unmarshal_xml(source).unwrap();
        assert_eq!(d.string_with_trailing_zeros(), "1.2300");
        assert_eq!(d.marshal_xml(), "1.23");
    }
    d.unmarshal_xml("<v>9007199254740993.00000000000000001</v>")
        .unwrap();
    assert_eq!(d.string(), "9007199254740993.00000000000000001");
    d.unmarshal_xml("<v>2</v><v>bad</v>").unwrap();
    assert_eq!(d, Decimal::new(2));
    d.unmarshal_xml_attr(" 3.400 ").unwrap();
    assert_eq!(d.string_with_trailing_zeros(), "3.400");
    assert!(d.unmarshal_xml_attr("&#49;").is_err());
}

#[test]
fn nullable_state_and_atomic_failure() {
    let mut n = NullDecimal::new(Decimal::new_with_prec(1230, 2));
    for text in [
        "<1>1</1>",
        "<v 1='x'>1</v>",
        "<v>bad</v>",
        "<v>",
        "<v>&unknown;</v>",
        "<v>&#0;</v>",
        "<v a='&bad;'>1</v>",
        "<v a='&#0;'/>",
        "<v>1</x>",
    ] {
        assert!(
            n.unmarshal_xml(text)
                .unwrap_err()
                .matches(ErrorKind::Unmarshal)
        );
        assert!(n.valid);
        assert_eq!(n.decimal.string_with_trailing_zeros(), "12.30");
    }
    n.unmarshal_xml("<v> \r\n </v>").unwrap();
    assert!(!n.valid && n.decimal.is_nil());
    assert_eq!(n.marshal_xml_attr(), None);
    assert_eq!(n.marshal_xml(), "");
    n = NullDecimal::new(Decimal::default());
    assert_eq!(n.marshal_xml_attr(), Some("0".into()));
    assert_eq!(n.marshal_xml(), "0");
    assert!(n.decimal.is_nil());
    let mut d = Decimal::new(1);
    d.unmarshal_xml("<v/>").unwrap();
    assert!(d.is_nil());
    assert_eq!(d.marshal_xml_attr(), "");
}
