use super::{
    CoreDecimal, Decimal, checked_i32, library_error, parameter_error, resource_error, tick,
};
use wasm_bindgen::prelude::*;
const PRECISION: i32 = 128;
const INPUT_SCALE: usize = 256;
const INPUT_BITS: u64 = 4096;
fn preflight(value: &CoreDecimal, prec: i32, exponential: bool) -> Result<(), &'static str> {
    if prec < 0 || (!exponential && value.sign() <= 0) {
        return Ok(());
    }
    if prec > PRECISION || value.scale() > INPUT_SCALE || value.coefficient().bits() > INPUT_BITS {
        return Err("playground logarithm/exponential exceeds precision or input budget");
    }
    if exponential {
        let magnitude = value.abs();
        // Match the core's proven round-to-zero branch before applying exp's magnitude cap.
        let zero_threshold =
            CoreDecimal::new_with_prec(25 * (prec as i64 + 1), 1).expect("fixed valid precision");
        if value.is_negative() && magnitude.gte(&zero_threshold) {
            return Ok(());
        }
        if magnitude.gt(&CoreDecimal::new(64)) {
            return Err("playground exponential magnitude exceeds 64");
        }
    }
    // The unchanged core refines with at most 8192 guard bits. These input/precision
    // caps keep rational reduction and fixed-point products bounded even on retries.
    Ok(())
}
#[wasm_bindgen]
impl Decimal {
    pub fn ln(&self) -> Result<Decimal, JsValue> {
        tick()?;
        preflight(&self.inner, self.inner.precision().max(20) as i32, false)
            .map_err(resource_error)?;
        self.inner
            .ln()
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn ln_with_prec(&self, prec: f64) -> Result<Decimal, JsValue> {
        tick()?;
        let prec = checked_i32(prec).map_err(parameter_error)?;
        preflight(&self.inner, prec, false).map_err(resource_error)?;
        self.inner
            .ln_with_prec(prec as isize)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn log2(&self) -> Result<Decimal, JsValue> {
        tick()?;
        preflight(&self.inner, self.inner.precision().max(20) as i32, false)
            .map_err(resource_error)?;
        self.inner
            .log2()
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn log2_with_prec(&self, prec: f64) -> Result<Decimal, JsValue> {
        tick()?;
        let prec = checked_i32(prec).map_err(parameter_error)?;
        preflight(&self.inner, prec, false).map_err(resource_error)?;
        self.inner
            .log2_with_prec(prec as isize)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn log10(&self) -> Result<Decimal, JsValue> {
        tick()?;
        preflight(&self.inner, self.inner.precision().max(20) as i32, false)
            .map_err(resource_error)?;
        self.inner
            .log10()
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn log10_with_prec(&self, prec: f64) -> Result<Decimal, JsValue> {
        tick()?;
        let prec = checked_i32(prec).map_err(parameter_error)?;
        preflight(&self.inner, prec, false).map_err(resource_error)?;
        self.inner
            .log10_with_prec(prec as isize)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn exp(&self) -> Result<Decimal, JsValue> {
        tick()?;
        preflight(&self.inner, self.inner.precision().max(20) as i32, true)
            .map_err(resource_error)?;
        self.inner
            .exp()
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn exp_with_prec(&self, prec: f64) -> Result<Decimal, JsValue> {
        tick()?;
        let prec = checked_i32(prec).map_err(parameter_error)?;
        preflight(&self.inner, prec, true).map_err(resource_error)?;
        self.inner
            .exp_with_prec(prec as isize)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn budgets_cover_input_precision_magnitude_and_safe_domains() {
        assert!(preflight(&CoreDecimal::new(2), 129, false).is_err());
        assert!(preflight(&CoreDecimal::new(65), 20, true).is_err());
        assert!(preflight(&CoreDecimal::new(-65), 128, true).is_err());
        assert!(preflight(&CoreDecimal::new(-65), 20, true).is_ok());
        assert!(preflight(&CoreDecimal::new(-2), 129, false).is_ok());
        assert!(preflight(&CoreDecimal::new(2), -1, true).is_ok());
        assert!(preflight(&CoreDecimal::new_from_string("1e-257").unwrap(), 20, false).is_err());
    }
}
