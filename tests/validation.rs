#![cfg(feature = "validation")]
use rs_decimal::{
    Decimal, ErrorKind,
    validation::{
        ValidationMessages, ValidationRule, default_validation_messages, validate_decimal,
    },
};
use std::collections::BTreeMap;
#[test]
fn exact_rules_presence_scale_and_invalid_configuration() {
    let value = Decimal::must_from_string("9007199254740993.00000000000000001");
    assert!(
        ValidationRule::parse("decimal_gt", "9007199254740993")
            .unwrap()
            .matches(&value)
    );
    assert!(
        !ValidationRule::parse("decimal_eq", "9007199254740993")
            .unwrap()
            .matches(&value)
    );
    let nil = Decimal::default();
    assert!(
        !ValidationRule::parse("decimal_required", "")
            .unwrap()
            .matches(&nil)
    );
    assert!(
        ValidationRule::parse("decimal_eq", "0")
            .unwrap()
            .matches(&nil)
    );
    assert!(
        !ValidationRule::parse("decimal_between", "0~0")
            .unwrap()
            .matches(&nil)
    );
    assert!(
        ValidationRule::parse("decimal_max_precision", "0")
            .unwrap()
            .matches(&nil)
    );
    let scaled = Decimal::must_from_string("1.2300");
    assert!(
        !ValidationRule::parse("decimal_max_precision", "2")
            .unwrap()
            .matches(&scaled)
    );
    for (tag, param) in [
        ("decimal_between", "2~1"),
        ("decimal_eq", "bad"),
        ("decimal_max_precision", "-1"),
        ("decimal_between", "1~2~3"),
        ("unknown", ""),
    ] {
        assert!(
            ValidationRule::parse(tag, param)
                .unwrap_err()
                .matches(ErrorKind::InvalidArgument)
        );
    }
}
#[test]
fn messages_are_owned_and_insertions_are_literal() {
    let rules = [
        ValidationRule::parse("decimal_required", "").unwrap(),
        ValidationRule::parse("decimal_gt", "0").unwrap(),
    ];
    let errors = validate_decimal("amount{1}", &Decimal::default(), &rules);
    assert_eq!(errors.len(), 2);
    let messages = ValidationMessages::new("zh-CN").translate(&errors);
    assert_eq!(messages, ["amount{1}为必填项", "amount{1}必须大于0"]);
    let custom = ValidationMessages::with_messages(BTreeMap::from([(
        "decimal_required".into(),
        "Required: {0}".into(),
    )]))
    .unwrap();
    assert_eq!(
        custom.translate(&errors),
        ["Required: amount{1}", "amount{1} must be greater than 0"]
    );
    assert!(
        ValidationMessages::with_messages(BTreeMap::from([(
            "decimal_required".into(),
            "{2}".into()
        )]))
        .is_err()
    );
    let mut first = default_validation_messages("zh-TW");
    first.clear();
    assert_eq!(
        default_validation_messages("zh-TW")["decimal_required"],
        "{0}為必填項"
    );
    assert_eq!(
        default_validation_messages("unknown"),
        default_validation_messages("en")
    );
}
