use super::{CoreDecimal, Decimal, limits, resource_error, tick};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn zero() -> Result<Decimal, JsValue> {
    tick()?;
    Ok(Decimal {
        inner: rs_decimal::zero(),
    })
}
#[wasm_bindgen]
pub fn one() -> Result<Decimal, JsValue> {
    tick()?;
    Ok(Decimal {
        inner: rs_decimal::one(),
    })
}
#[wasm_bindgen]
pub fn ten() -> Result<Decimal, JsValue> {
    tick()?;
    Ok(Decimal {
        inner: rs_decimal::ten(),
    })
}
#[wasm_bindgen]
pub fn hundred() -> Result<Decimal, JsValue> {
    tick()?;
    Ok(Decimal {
        inner: rs_decimal::hundred(),
    })
}
#[wasm_bindgen]
pub fn min(a: &Decimal, b: &Decimal) -> Result<Decimal, JsValue> {
    tick()?;
    Ok(Decimal {
        inner: rs_decimal::min(&a.inner, &b.inner),
    })
}
#[wasm_bindgen]
pub fn max(a: &Decimal, b: &Decimal) -> Result<Decimal, JsValue> {
    tick()?;
    Ok(Decimal {
        inner: rs_decimal::max(&a.inner, &b.inner),
    })
}
#[wasm_bindgen]
pub fn between(v: &Decimal, lower: &Decimal, upper: &Decimal) -> Result<bool, JsValue> {
    tick()?;
    Ok(rs_decimal::between(&v.inner, &lower.inner, &upper.inner))
}
#[wasm_bindgen]
impl Decimal {
    pub fn must_from_string(text: &str) -> Result<Decimal, JsValue> {
        tick()?;
        limits::input(text).map_err(resource_error)?;
        // Do not turn the core's panic into a recoverable library Result.
        Ok(Decimal {
            inner: CoreDecimal::must_from_string(text),
        })
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn invalid_must_parse_panics_with_the_original_library_error() {
        let panic =
            std::panic::catch_unwind(|| rs_decimal::Decimal::must_from_string("bad")).unwrap_err();
        assert!(panic.downcast_ref::<rs_decimal::DecimalError>().is_some());
    }
}
