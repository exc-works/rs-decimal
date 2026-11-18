use super::{
    CoreDecimal, Decimal, checked_i32, constructors, library_error, limits, parameter_error,
    resource_error, tick,
};
use wasm_bindgen::prelude::*;
const MAX_DEGREE: i64 = 128;
fn preflight(value: &CoreDecimal, degree: i64, prec: i32) -> Result<(), &'static str> {
    // Core rejects these domains before any growing arithmetic allocation.
    if prec < 0 || degree <= 0 || (value.is_negative() && degree % 2 == 0) {
        return Ok(());
    }
    if prec as usize > limits::SCALE {
        return Err("playground root exceeds output scale budget");
    }
    if degree == 1 {
        return limits::rescale(value, prec);
    }
    if value.is_zero() || value.abs().equal(&CoreDecimal::new(1)) {
        return Ok(());
    }
    if degree > MAX_DEGREE {
        return Err("playground root exceeds degree budget");
    }
    let shift = degree * prec as i64 - value.scale() as i64;
    // Bound scaled numerator/denominator, Newton powers, and midpoint certification.
    // ceil(bits / degree) contributes at most another degree; reserve three degrees
    // for the doubled candidate and its power, plus small carry/denominator slack.
    let bits = value.coefficient().bits() + 4 * shift.unsigned_abs() + 3 * degree as u64 + 4;
    if bits > limits::COEFFICIENT_BITS {
        return Err("playground root exceeds intermediate coefficient budget");
    }
    Ok(())
}
#[wasm_bindgen]
impl Decimal {
    pub fn sqrt(&self) -> Result<Decimal, JsValue> {
        tick()?;
        preflight(&self.inner, 2, self.inner.precision().max(20) as i32).map_err(resource_error)?;
        self.inner
            .sqrt()
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn sqrt_with_prec(&self, prec: f64) -> Result<Decimal, JsValue> {
        tick()?;
        let prec = checked_i32(prec).map_err(parameter_error)?;
        preflight(&self.inner, 2, prec).map_err(resource_error)?;
        self.inner
            .sqrt_with_prec(prec as isize)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn approx_root(&self, degree: &str) -> Result<Decimal, JsValue> {
        tick()?;
        let degree = constructors::signed(degree)?;
        preflight(&self.inner, degree, self.inner.precision().max(20) as i32)
            .map_err(resource_error)?;
        self.inner
            .approx_root(degree)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn approx_root_with_prec(&self, degree: &str, prec: f64) -> Result<Decimal, JsValue> {
        tick()?;
        let degree = constructors::signed(degree)?;
        let prec = checked_i32(prec).map_err(parameter_error)?;
        preflight(&self.inner, degree, prec).map_err(resource_error)?;
        self.inner
            .approx_root_with_prec(degree, prec as isize)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roots_bound_intermediates_and_preserve_fast_domains() {
        let two = CoreDecimal::new(2);
        assert!(preflight(&two, 129, 20).is_err());
        assert!(preflight(&two, 128, 1000).is_err());
        assert!(preflight(&two, 2, 1000).is_ok());
        assert!(preflight(&CoreDecimal::new(-2), 2, 1001).is_ok());
        assert!(preflight(&two, 0, 1001).is_ok());
        assert!(preflight(&two, 2, -1).is_ok());
        assert!(preflight(&CoreDecimal::new(-1), i64::MAX, 1000).is_ok());
        assert!(preflight(&CoreDecimal::default(), i64::MAX, 1000).is_ok());
    }
}
