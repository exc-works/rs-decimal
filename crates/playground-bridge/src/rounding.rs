use super::{
    CoreDecimal, Decimal, RoundingMode, checked_i32, library_error, limits, parameter_error,
    resource_error, tick,
};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn rounding_mode_from_raw(value: f64) -> Result<i32, JsValue> {
    tick()?;
    RoundingMode::from_raw(checked_i32(value).map_err(parameter_error)?)
        .map(|mode| mode as i32)
        .map_err(library_error)
}
#[wasm_bindgen]
impl Decimal {
    pub fn ceil(&self) -> Result<Decimal, JsValue> {
        tick()?;
        Ok(Self {
            inner: self.inner.ceil(),
        })
    }
    pub fn floor(&self) -> Result<Decimal, JsValue> {
        tick()?;
        Ok(Self {
            inner: self.inner.floor(),
        })
    }
    pub fn round(&self) -> Result<Decimal, JsValue> {
        tick()?;
        Ok(Self {
            inner: self.inner.round(),
        })
    }
    pub fn truncate(&self) -> Result<Decimal, JsValue> {
        tick()?;
        Ok(Self {
            inner: self.inner.truncate(),
        })
    }
    pub fn strip_trailing_zeros(&self) -> Result<Decimal, JsValue> {
        tick()?;
        Ok(Self {
            inner: self.inner.strip_trailing_zeros(),
        })
    }
    pub fn ceil_with_prec(&self, prec: f64) -> Result<Decimal, JsValue> {
        tick()?;
        let prec = checked_i32(prec).map_err(parameter_error)?;
        limits::rescale(&self.inner, prec).map_err(resource_error)?;
        self.inner
            .ceil_with_prec(prec as isize)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn floor_with_prec(&self, prec: f64) -> Result<Decimal, JsValue> {
        tick()?;
        let prec = checked_i32(prec).map_err(parameter_error)?;
        limits::rescale(&self.inner, prec).map_err(resource_error)?;
        self.inner
            .floor_with_prec(prec as isize)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn round_with_prec(&self, prec: f64) -> Result<Decimal, JsValue> {
        tick()?;
        let prec = checked_i32(prec).map_err(parameter_error)?;
        limits::rescale(&self.inner, prec).map_err(resource_error)?;
        self.inner
            .round_with_prec(prec as isize)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn truncate_with_prec(&self, prec: f64) -> Result<Decimal, JsValue> {
        tick()?;
        let prec = checked_i32(prec).map_err(parameter_error)?;
        limits::rescale(&self.inner, prec).map_err(resource_error)?;
        self.inner
            .truncate_with_prec(prec as isize)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn rescale_down(&self, prec: f64) -> Result<Decimal, JsValue> {
        tick()?;
        let prec = checked_i32(prec).map_err(parameter_error)?;
        limits::rescale(&self.inner, prec).map_err(resource_error)?;
        self.inner
            .rescale_down(prec as isize)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn shift(&self, places: f64) -> Result<Decimal, JsValue> {
        tick()?;
        let places = checked_i32(places).map_err(parameter_error)?;
        shift_budget(&self.inner, places).map_err(resource_error)?;
        self.inner
            .shift(places as isize)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn significant_figures(&self, figures: f64, mode: f64) -> Result<Decimal, JsValue> {
        tick()?;
        let figures = checked_i32(figures).map_err(parameter_error)?;
        let mode = RoundingMode::from_raw(checked_i32(mode).map_err(parameter_error)?)
            .map_err(library_error)?;
        // Positive figures never add scale. Reserve a possible rounding carry
        // before core allocation; nonpositive counts retain InvalidArgument.
        if figures > 0
            && !self.inner.is_zero()
            && (figures as usize) < self.inner.coefficient().magnitude().to_string().len()
            && self.inner.coefficient().bits() + 4 > limits::COEFFICIENT_BITS
        {
            return Err(resource_error(
                "playground significant-figure carry exceeds coefficient budget",
            ));
        }
        self.inner
            .significant_figures(figures as isize, mode)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
}
fn shift_budget(value: &CoreDecimal, places: i32) -> Result<(), &'static str> {
    let target = value.scale() as i64 - places as i64;
    // The core returns Overflow here before allocating; preserve that error.
    if target > i32::MAX as i64 {
        return Ok(());
    }
    if target > limits::SCALE as i64
        || (target < 0
            && !value.is_zero()
            && value.coefficient().bits() + target.unsigned_abs() * 4 > limits::COEFFICIENT_BITS)
    {
        return Err("playground shift exceeds scale or coefficient budget");
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shift_preflight_bounds_growth_and_preserves_core_overflow() {
        let value = CoreDecimal::new(1);
        assert!(shift_budget(&value, i32::MAX).is_err());
        assert!(shift_budget(&value, -1001).is_err());
        assert!(shift_budget(&value, i32::MIN).is_ok());
        assert!(value.shift(i32::MIN as isize).is_err());
        assert!(shift_budget(&CoreDecimal::new(0), i32::MAX).is_ok());
    }
}
