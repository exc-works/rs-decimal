//! Exact JSON configuration with application-level validation.
use rs_decimal::{Decimal, NullDecimal, validation::ValidationRule};

#[derive(serde::Deserialize)]
struct Settings {
    #[serde(deserialize_with = "rs_decimal::config::deserialize_json_decimal")]
    amount: Decimal,
    #[serde(
        default,
        deserialize_with = "rs_decimal::config::deserialize_json_nullable"
    )]
    limit: NullDecimal,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings: Settings =
        serde_json::from_str(r#"{"amount":9007199254740993.00000000000000001,"limit":null}"#)?;
    assert_eq!(
        settings.amount.string(),
        "9007199254740993.00000000000000001"
    );
    assert!(!settings.limit.valid);
    let positive = ValidationRule::parse("decimal_positive", "")?;
    assert!(positive.matches(&settings.amount));
    println!("amount = {}", settings.amount);
    Ok(())
}
