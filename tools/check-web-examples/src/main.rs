use rs_decimal::{BigInt, Decimal};
fn snapshot(value: &Decimal) -> serde_json::Value {
    serde_json::json!({"coefficient":value.coefficient().to_string(),"scale":value.scale().to_string(),"initialized":!value.is_nil(),"string":value.string(),"string_with_trailing_zeros":value.string_with_trailing_zeros()})
}
include!(concat!(env!("OUT_DIR"), "/examples.rs"));
fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_examples()
}
