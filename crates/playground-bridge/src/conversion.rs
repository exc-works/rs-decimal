use super::{
    CoreDecimal, Decimal, RoundingMode, checked_i32, constructors, library_error, limits,
    parameter_error, resource_error, tick,
};
use rs_decimal::{BigInt, BigRational};
use wasm_bindgen::prelude::*;
fn rational(numerator: &str, denominator: &str) -> Result<BigRational, JsValue> {
    let n = constructors::big(numerator)?;
    let d = constructors::big(denominator)?;
    if d == BigInt::from(0) {
        return Err(parameter_error("rational denominator must be nonzero"));
    }
    Ok(BigRational::new(n, d))
}
fn exact_rational_budget(value: &BigRational) -> Result<(), &'static str> {
    if value.numer() == &BigInt::from(0) {
        return Ok(());
    }
    let mut den = value.denom().clone();
    let twos = den.trailing_zeros().unwrap_or(0) as usize;
    if twos > limits::SCALE {
        return Err("playground rational exceeds scale budget");
    }
    den >>= twos;
    let five = BigInt::from(5);
    let mut fives = 0;
    while &den % &five == BigInt::from(0) {
        fives += 1;
        if fives > limits::SCALE {
            return Err("playground rational exceeds scale budget");
        }
        den /= &five;
    }
    // A remaining denominator factor is rejected by the core before coefficient growth.
    if den != BigInt::from(1) {
        return Ok(());
    }
    let growth = if twos > fives {
        (twos - fives) * 3
    } else {
        fives - twos
    };
    if value.numer().bits() + growth as u64 > limits::COEFFICIENT_BITS {
        return Err("playground rational exceeds coefficient budget");
    }
    Ok(())
}
fn tuple(value: JsValue, exact: bool) -> js_sys::Array {
    let result = js_sys::Array::new();
    result.push(&value);
    result.push(&JsValue::from_bool(exact));
    result
}
#[wasm_bindgen]
impl Decimal {
    pub fn new_from_float32(value: f64) -> Result<Decimal, JsValue> {
        tick()?;
        CoreDecimal::new_from_float32(value as f32)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn new_from_float64(value: f64) -> Result<Decimal, JsValue> {
        tick()?;
        CoreDecimal::new_from_float64(value)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn new_from_big_rat(numerator: &str, denominator: &str) -> Result<Decimal, JsValue> {
        tick()?;
        let value = rational(numerator, denominator)?;
        exact_rational_budget(&value).map_err(resource_error)?;
        CoreDecimal::new_from_big_rat(&value)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn new_from_big_rat_with_prec(
        numerator: &str,
        denominator: &str,
        prec: f64,
        mode: f64,
    ) -> Result<Decimal, JsValue> {
        tick()?;
        let value = rational(numerator, denominator)?;
        let prec = checked_i32(prec).map_err(parameter_error)?;
        let mode = RoundingMode::from_raw(checked_i32(mode).map_err(parameter_error)?)
            .map_err(library_error)?;
        if prec >= 0
            && (prec as usize > limits::SCALE
                || value.numer().bits() + prec as u64 * 4 > limits::COEFFICIENT_BITS)
        {
            return Err(resource_error(
                "playground rational rounding exceeds scale or coefficient budget",
            ));
        }
        CoreDecimal::new_from_big_rat_with_prec(&value, prec as isize, mode)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn big_rat_text(&self) -> Result<js_sys::Array, JsValue> {
        tick()?;
        let value = self.inner.big_rat();
        let result = js_sys::Array::new();
        result.push(&JsValue::from_str(&value.numer().to_string()));
        result.push(&JsValue::from_str(&value.denom().to_string()));
        Ok(result)
    }
    pub fn bit_len(&self) -> Result<u32, JsValue> {
        tick()?;
        Ok(self.inner.bit_len() as u32)
    }
    pub fn int64_text(&self) -> Result<js_sys::Array, JsValue> {
        tick()?;
        let (value, exact) = self.inner.int64();
        Ok(tuple(JsValue::from_str(&value.to_string()), exact))
    }
    pub fn uint64_text(&self) -> Result<js_sys::Array, JsValue> {
        tick()?;
        let (value, exact) = self.inner.uint64();
        Ok(tuple(JsValue::from_str(&value.to_string()), exact))
    }
    pub fn float32(&self) -> Result<js_sys::Array, JsValue> {
        tick()?;
        let (value, exact) = self.inner.float32();
        Ok(tuple(JsValue::from_f64(value as f64), exact))
    }
    pub fn float64(&self) -> Result<js_sys::Array, JsValue> {
        tick()?;
        let (value, exact) = self.inner.float64();
        Ok(tuple(JsValue::from_f64(value), exact))
    }
}
#[wasm_bindgen]
pub fn most_significant_bit(value: &str) -> Result<u32, JsValue> {
    tick()?;
    rs_decimal::most_significant_bit(&constructors::big(value)?)
        .map(|value| value as u32)
        .map_err(library_error)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_rational_preflight_bounds_factor_growth_after_reduction() {
        let one = BigInt::from(1);
        assert!(exact_rational_budget(&BigRational::new(one.clone(), &one << 1000)).is_ok());
        assert!(exact_rational_budget(&BigRational::new(one.clone(), &one << 1001)).is_err());
        assert!(
            exact_rational_budget(&BigRational::new(one.clone(), BigInt::from(5).pow(1001)))
                .is_err()
        );
        assert!(exact_rational_budget(&BigRational::new(one.clone(), BigInt::from(3))).is_ok());
        assert!(
            exact_rational_budget(&BigRational::new(&one << 15000, BigInt::from(5).pow(1000)))
                .is_ok()
        );
        assert!(
            exact_rational_budget(&BigRational::new((&one << 15000) + 1, &one << 1000)).is_err()
        );
    }
}
