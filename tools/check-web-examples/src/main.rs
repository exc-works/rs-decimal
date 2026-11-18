use rs_decimal::{BigInt, Decimal};
trait ExampleState {
    fn state(&self) -> serde_json::Value;
}
impl ExampleState for Decimal {
    fn state(&self) -> serde_json::Value {
        serde_json::json!({"coefficient":self.coefficient().to_string(),"scale":self.scale().to_string(),"initialized":!self.is_nil(),"string":self.string(),"string_with_trailing_zeros":self.string_with_trailing_zeros()})
    }
}
impl ExampleState for serde_json::Value {
    fn state(&self) -> serde_json::Value {
        self.clone()
    }
}
fn snapshot(value: &impl ExampleState) -> serde_json::Value {
    value.state()
}
include!(concat!(env!("OUT_DIR"), "/examples.rs"));
fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_examples()
}
