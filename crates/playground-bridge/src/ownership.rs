use super::{
    CoreDecimal, Decimal, arithmetic, constructors, library_error, limits, parameter_error,
    resource_error, tick,
};
use wasm_bindgen::prelude::*;

fn power_budget(value: &CoreDecimal, power: i64) -> Result<(), &'static str> {
    if power == 0 || value.is_zero() {
        return Ok(());
    }
    let base = value.strip_trailing_zeros();
    if base.scale() == 0 && base.coefficient().bits() == 1 {
        return Ok(()); // The core handles +/-1 without exponentiation.
    }
    let magnitude = power.unsigned_abs() as u128;
    let size = (base.scale() as u128)
        .max(base.coefficient().bits() as u128)
        .max(1);
    if magnitude > i32::MAX as u128 / size {
        return Ok(()); // Preserve the core's pre-allocation Overflow.
    }
    let scale = base.scale() as u128 * magnitude;
    let bits = base.coefficient().bits() as u128 * magnitude;
    let growth = (value.scale() as u128).saturating_sub(scale) * 4;
    if scale > limits::SCALE as u128 || bits + growth > limits::COEFFICIENT_BITS as u128 {
        return Err("playground power exceeds intermediate scale or coefficient budget");
    }
    // Reciprocal division may build 10^(original scale + powered scale).
    if power < 0 && 1 + 4 * (value.scale() as u128 + scale) > limits::COEFFICIENT_BITS as u128 {
        return Err("playground reciprocal exceeds coefficient budget");
    }
    Ok(())
}
fn remainder_budget(a: &CoreDecimal, b: &CoreDecimal) -> Result<(), &'static str> {
    if b.is_zero() || a.is_zero() {
        return Ok(());
    }
    arithmetic::aligned(a, b)
}
#[wasm_bindgen]
impl Decimal {
    pub fn power(&self, power: &str) -> Result<Decimal, JsValue> {
        tick()?;
        let power = constructors::signed(power)?;
        power_budget(&self.inner, power).map_err(resource_error)?;
        self.inner
            .power(power)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn quo_rem(&self, other: &Decimal) -> Result<js_sys::Array, JsValue> {
        tick()?;
        remainder_budget(&self.inner, &other.inner).map_err(resource_error)?;
        let (quotient, remainder) = self.inner.quo_rem(&other.inner).map_err(library_error)?;
        let result = js_sys::Array::new();
        result.push(&JsValue::from(Self { inner: quotient }));
        result.push(&JsValue::from(Self { inner: remainder }));
        Ok(result)
    }
    pub fn modulo(&self, other: &Decimal) -> Result<Decimal, JsValue> {
        tick()?;
        remainder_budget(&self.inner, &other.inner).map_err(resource_error)?;
        self.inner
            .modulo(&other.inner)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    #[wasm_bindgen(js_name = mod)]
    pub fn mod_value(&self, other: &Decimal) -> Result<Decimal, JsValue> {
        tick()?;
        remainder_budget(&self.inner, &other.inner).map_err(resource_error)?;
        self.inner
            .r#mod(&other.inner)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn remainder_text(&self) -> Result<js_sys::Array, JsValue> {
        tick()?;
        let (integer, remainder) = self.inner.remainder();
        let result = js_sys::Array::new();
        result.push(&JsValue::from_str(&integer.to_string()));
        result.push(&JsValue::from_str(&remainder.to_string()));
        Ok(result)
    }
    pub fn into_coefficient_text(self) -> Result<String, JsValue> {
        tick()?;
        Ok(self.inner.into_coefficient().to_string())
    }
    pub fn try_from_coefficient(coefficient: &str, scale: f64) -> Result<Decimal, JsValue> {
        tick()?;
        if !scale.is_finite() || scale.fract() != 0.0 || !(0.0..=u32::MAX as f64).contains(&scale) {
            return Err(parameter_error("scale must fit wasm32 usize"));
        }
        let coefficient = constructors::big(coefficient)?;
        if scale <= i32::MAX as f64 && scale > limits::SCALE as f64 {
            return Err(resource_error(
                "playground coefficient scale exceeds budget",
            ));
        }
        // Explicit target-width check also keeps native wrapper builds deterministic.
        if scale > i32::MAX as f64 {
            return Err(library_error(rs_decimal::DecimalError::new(
                rs_decimal::ErrorKind::Overflow,
                "decimal scale exceeds isize::MAX",
            )));
        }
        CoreDecimal::try_from_coefficient(coefficient, scale as usize)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn add_owned(self, other: &Decimal) -> Result<Decimal, JsValue> {
        tick()?;
        arithmetic::aligned(&self.inner, &other.inner).map_err(resource_error)?;
        Ok(Self {
            inner: self.inner.add_owned(&other.inner),
        })
    }
    pub fn sub_owned(self, other: &Decimal) -> Result<Decimal, JsValue> {
        tick()?;
        arithmetic::aligned(&self.inner, &other.inner).map_err(resource_error)?;
        Ok(Self {
            inner: self.inner.sub_owned(&other.inner),
        })
    }
    pub fn max_ref_select_self(&self, other: &Decimal) -> Result<bool, JsValue> {
        tick()?;
        Ok(std::ptr::eq(self.inner.max_ref(&other.inner), &self.inner))
    }
    pub fn min_ref_select_self(&self, other: &Decimal) -> Result<bool, JsValue> {
        tick()?;
        Ok(std::ptr::eq(self.inner.min_ref(&other.inner), &self.inner))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn power_checks_intermediates_but_retains_core_fast_paths() {
        assert!(power_budget(&CoreDecimal::new(2), 10000).is_err());
        assert!(power_budget(&CoreDecimal::new_from_string("0.1").unwrap(), 1001).is_err());
        assert!(power_budget(&CoreDecimal::new(2), i64::MAX).is_ok());
        assert!(power_budget(&CoreDecimal::new(-1), i64::MIN).is_ok());
        assert!(power_budget(&CoreDecimal::new(0), -1).is_ok());
        assert!(power_budget(&CoreDecimal::new_from_string("2.00").unwrap(), -3).is_ok());
    }
}
