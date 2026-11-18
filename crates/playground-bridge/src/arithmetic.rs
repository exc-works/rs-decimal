use super::{
    CoreDecimal, Decimal, RoundingMode, checked_i32, constructors, library_error, limits,
    parameter_error, resource_error, tick,
};
use wasm_bindgen::prelude::*;
fn mode(value: f64) -> Result<RoundingMode, JsValue> {
    RoundingMode::from_raw(checked_i32(value).map_err(parameter_error)?).map_err(library_error)
}
fn bound(bits: u64) -> Result<(), &'static str> {
    if bits > limits::COEFFICIENT_BITS {
        Err("playground arithmetic exceeds coefficient budget")
    } else {
        Ok(())
    }
}
fn aligned(a: &CoreDecimal, b: &CoreDecimal) -> Result<(), &'static str> {
    let scale = a.scale().max(b.scale());
    let a_bits = a.coefficient().bits() + (scale - a.scale()) as u64 * 4;
    let b_bits = b.coefficient().bits() + (scale - b.scale()) as u64 * 4;
    bound(a_bits.max(b_bits) + 1)
}
fn product(a: &CoreDecimal, b: &CoreDecimal, prec: i32) -> Result<(), &'static str> {
    if prec < 0 {
        return Ok(());
    }
    let prec = prec as usize;
    if prec > limits::SCALE {
        return Err("playground product exceeds scale budget");
    }
    let growth = prec.saturating_sub(a.scale() + b.scale()) as u64 * 4;
    bound(a.coefficient().bits() + b.coefficient().bits() + growth)
}
fn quotient(a: &CoreDecimal, b: &CoreDecimal, prec: i32) -> Result<(), &'static str> {
    // Core checks invalid precision and division by zero before allocation.
    if prec < 0 || b.is_zero() {
        return Ok(());
    }
    if prec as usize > limits::SCALE {
        return Err("playground quotient exceeds scale budget");
    }
    if a.is_zero() {
        return Ok(());
    }
    let shift = prec as i64 - a.scale() as i64 + b.scale() as i64;
    if shift >= 0 {
        bound(a.coefficient().bits() + shift as u64 * 4)
    } else {
        bound(b.coefficient().bits() + shift.unsigned_abs() * 4)
    }
}
#[wasm_bindgen]
impl Decimal {
    pub fn add(&self, other: &Decimal) -> Result<Decimal, JsValue> {
        tick()?;
        aligned(&self.inner, &other.inner).map_err(resource_error)?;
        Ok(Self {
            inner: CoreDecimal::add(&self.inner, &other.inner),
        })
    }
    pub fn sub(&self, other: &Decimal) -> Result<Decimal, JsValue> {
        tick()?;
        aligned(&self.inner, &other.inner).map_err(resource_error)?;
        Ok(Self {
            inner: CoreDecimal::sub(&self.inner, &other.inner),
        })
    }
    pub fn safe_add(&self, other: &Decimal) -> Result<Decimal, JsValue> {
        tick()?;
        aligned(&self.inner, &other.inner).map_err(resource_error)?;
        CoreDecimal::safe_add(&self.inner, &other.inner)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn safe_sub(&self, other: &Decimal) -> Result<Decimal, JsValue> {
        tick()?;
        aligned(&self.inner, &other.inner).map_err(resource_error)?;
        CoreDecimal::safe_sub(&self.inner, &other.inner)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn add_raw(&self, value: &str) -> Result<Decimal, JsValue> {
        tick()?;
        let value = constructors::signed(value)?;
        bound(self.inner.coefficient().bits().max(64) + 1).map_err(resource_error)?;
        Ok(Self {
            inner: self.inner.add_raw(value),
        })
    }
    pub fn sub_raw(&self, value: &str) -> Result<Decimal, JsValue> {
        tick()?;
        let value = constructors::signed(value)?;
        bound(self.inner.coefficient().bits().max(64) + 1).map_err(resource_error)?;
        Ok(Self {
            inner: self.inner.sub_raw(value),
        })
    }
    pub fn mul(&self, other: &Decimal, rounding: f64) -> Result<Decimal, JsValue> {
        tick()?;
        let mode = mode(rounding)?;
        product(
            &self.inner,
            &other.inner,
            self.inner.scale().max(other.inner.scale()) as i32,
        )
        .map_err(resource_error)?;
        self.inner
            .mul(&other.inner, mode)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn mul_down(&self, other: &Decimal) -> Result<Decimal, JsValue> {
        tick()?;
        product(
            &self.inner,
            &other.inner,
            self.inner.scale().max(other.inner.scale()) as i32,
        )
        .map_err(resource_error)?;
        Ok(Self {
            inner: self.inner.mul_down(&other.inner),
        })
    }
    pub fn mul_with_prec(
        &self,
        other: &Decimal,
        prec: f64,
        rounding: f64,
    ) -> Result<Decimal, JsValue> {
        tick()?;
        let prec = checked_i32(prec).map_err(parameter_error)?;
        let mode = mode(rounding)?;
        product(&self.inner, &other.inner, prec).map_err(resource_error)?;
        self.inner
            .mul_with_prec(&other.inner, prec as isize, mode)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn quo(&self, other: &Decimal, rounding: f64) -> Result<Decimal, JsValue> {
        tick()?;
        let mode = mode(rounding)?;
        quotient(
            &self.inner,
            &other.inner,
            self.inner.scale().max(other.inner.scale()) as i32,
        )
        .map_err(resource_error)?;
        self.inner
            .quo(&other.inner, mode)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn quo_down(&self, other: &Decimal) -> Result<Decimal, JsValue> {
        tick()?;
        quotient(
            &self.inner,
            &other.inner,
            self.inner.scale().max(other.inner.scale()) as i32,
        )
        .map_err(resource_error)?;
        self.inner
            .quo_down(&other.inner)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn quo_with_prec(
        &self,
        other: &Decimal,
        prec: f64,
        rounding: f64,
    ) -> Result<Decimal, JsValue> {
        tick()?;
        let prec = checked_i32(prec).map_err(parameter_error)?;
        let mode = mode(rounding)?;
        quotient(&self.inner, &other.inner, prec).map_err(resource_error)?;
        self.inner
            .quo_with_prec(&other.inner, prec as isize, mode)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }
    pub fn is_integer(&self) -> Result<bool, JsValue> {
        tick()?;
        Ok(self.inner.is_integer())
    }
    pub fn has_fraction(&self) -> Result<bool, JsValue> {
        tick()?;
        Ok(self.inner.has_fraction())
    }
    pub fn int_part_text(&self) -> Result<String, JsValue> {
        tick()?;
        Ok(self.inner.int_part().to_string())
    }
    pub fn must_non_negative(&self) -> Result<Decimal, JsValue> {
        tick()?;
        Ok(Self {
            inner: self.inner.must_non_negative(),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn allocation_preflight_covers_alignment_products_and_division() {
        let large = CoreDecimal::new_from_string(&"9".repeat(4096)).unwrap();
        let scaled = CoreDecimal::new_with_prec(1, 1000).unwrap();
        assert!(aligned(&large, &scaled).is_err());
        assert!(product(&large, &large, 0).is_err());
        assert!(quotient(&large, &scaled, 1000).is_err());
        assert!(quotient(&scaled, &large, 0).is_err());
        assert!(quotient(&large, &CoreDecimal::new(0), i32::MAX).is_ok());
        assert!(product(&large, &large, -1).is_ok());
    }
    #[test]
    fn negative_requirement_retains_the_core_panic() {
        assert!(std::panic::catch_unwind(|| CoreDecimal::new(-1).must_non_negative()).is_err());
    }
}
