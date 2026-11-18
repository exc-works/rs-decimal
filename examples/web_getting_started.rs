use rs_decimal::{Decimal, RoundingMode};

fn main() -> Result<(), rs_decimal::DecimalError> {
    let price = Decimal::new_from_string("12.350")?;
    let total = price.mul_exact(&Decimal::new(3))?;
    assert_eq!(total.string(), "37.05");
    assert_eq!(total.string_with_trailing_zeros(), "37.050");
    let rounded = total.rescale(1, RoundingMode::HalfEven)?;
    assert_eq!(rounded.string(), "37");
    Ok(())
}
