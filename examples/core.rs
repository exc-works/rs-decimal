fn main() -> Result<(), rs_decimal::DecimalError> {
    use rs_decimal::{Decimal, GoFormat, RoundingMode};
    let price = Decimal::must_from_string("12.350");
    assert_eq!(price.string_with_trailing_zeros(), "12.350");
    let total = price.mul_exact(&Decimal::new(3))?;
    assert_eq!(total.string(), "37.05");
    let third = Decimal::new(1).quo_with_prec(&Decimal::new(3), 2, RoundingMode::HalfEven)?;
    assert_eq!(third.string_with_trailing_zeros(), "0.33");
    assert_eq!(Decimal::new(2).sqrt_with_prec(5)?.string(), "1.41421");
    assert_eq!(
        price.format_go(GoFormat {
            verb: 'f',
            precision: Some(2),
            ..GoFormat::default()
        })?,
        "12.35"
    );
    Ok(())
}
