use rs_decimal::Decimal;

fn main() -> Result<(), rs_decimal::DecimalError> {
    // #region calculate
    let price = Decimal::new_from_string("4.50")?;
    let quantity = Decimal::new(2);
    let subtotal = price.mul_exact(&quantity)?;
    let delivery = Decimal::new_from_string("1.25")?;
    let total = subtotal.add(&delivery);
    // #endregion calculate

    // #region print
    println!("{}", total.string_with_trailing_zeros()); // 10.25
    // #endregion print
    assert_eq!(subtotal.string_with_trailing_zeros(), "9.00");
    assert_eq!(total.string_with_trailing_zeros(), "10.25");
    Ok(())
}
