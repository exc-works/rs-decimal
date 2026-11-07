#![cfg(feature = "config")]
use rs_decimal::{
    Decimal, NullDecimal,
    config::{ConfigValue, decode_decimal, decode_nullable},
};
use serde::Deserialize;
#[derive(Deserialize)]
struct Settings {
    #[serde(deserialize_with = "rs_decimal::config::deserialize_json_decimal")]
    amount: Decimal,
    #[serde(
        default,
        deserialize_with = "rs_decimal::config::deserialize_json_nullable"
    )]
    limit: NullDecimal,
}
#[test]
fn serde_json_preserves_tokens_and_field_presence() {
    let config: Settings =
        serde_json::from_str(r#"{"amount":9007199254740993.00000000000000001,"limit":"1.2300"}"#)
            .unwrap();
    assert_eq!(config.amount.string(), "9007199254740993.00000000000000001");
    assert_eq!(config.limit.decimal.string_with_trailing_zeros(), "1.2300");
    assert!(config.limit.valid);
    for optional in ["", ",\"limit\":null", ",\"limit\":\"\""] {
        let config: Settings =
            serde_json::from_str(&format!("{{\"amount\":1e-40{optional}}}")).unwrap();
        assert_eq!(config.amount.precision(), 40);
        assert!(!config.limit.valid && config.limit.decimal.is_nil());
    }
    for raw in [
        r#"{}"#,
        r#"{"amount":null}"#,
        r#"{"amount":""}"#,
        r#"{"amount":true}"#,
        r#"{"amount":{ "$serde_json::private::Number":"123"}}"#,
        r#"{"amount":1,"limit":" "}"#,
    ] {
        assert!(serde_json::from_str::<Settings>(raw).is_err());
    }
}
#[test]
fn typed_hook_clones_states_and_rejects_nonfinite() {
    assert!(decode_decimal(ConfigValue::Null).is_err());
    assert!(decode_decimal(ConfigValue::Text("")).is_err());
    assert!(!decode_nullable(ConfigValue::Text("")).unwrap().valid);
    assert!(!decode_nullable(ConfigValue::Bytes(&[])).unwrap().valid);
    assert!(decode_nullable(ConfigValue::Number("")).is_err());
    assert!(decode_decimal(ConfigValue::Float64(f64::NAN)).is_err());
    assert_eq!(
        decode_decimal(ConfigValue::Unsigned(u128::MAX))
            .unwrap()
            .string(),
        u128::MAX.to_string()
    );
    let nil = Decimal::default();
    let valid_nil = decode_nullable(ConfigValue::Decimal(&nil)).unwrap();
    assert!(valid_nil.valid && valid_nil.decimal.is_nil());
    assert!(decode_decimal(ConfigValue::Nullable(&NullDecimal::default())).is_err());
    let original = Decimal::must_from_string("1.2300");
    let mut copy = decode_decimal(ConfigValue::Decimal(&original)).unwrap();
    copy.unmarshal_text(b"8").unwrap();
    assert_eq!(original.string_with_trailing_zeros(), "1.2300");
}
