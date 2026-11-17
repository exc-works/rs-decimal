//! Minimal browser feasibility bridge. Not yet an untrusted script runner.
use rs_decimal::{Decimal as CoreDecimal, DecimalError, RoundingMode};
use wasm_bindgen::prelude::*;

/// Browser wrapper retaining the original decimal value and representation.
#[wasm_bindgen]
pub struct Decimal {
    inner: CoreDecimal,
}

fn library_error(error: DecimalError) -> JsError {
    JsError::new(&error.to_string())
}

fn checked_i32(value: f64) -> Result<i32, &'static str> {
    if !value.is_finite()
        || value.fract() != 0.0
        || value < i32::MIN as f64
        || value > i32::MAX as f64
    {
        Err("bridge parameter must be an integer in the wasm32 signed range")
    } else {
        Ok(value as i32)
    }
}

#[wasm_bindgen]
impl Decimal {
    /// Parses exact decimal text using the original Rust constructor.
    pub fn new_from_string(text: &str) -> Result<Decimal, JsError> {
        CoreDecimal::new_from_string(text)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }

    /// Creates uninitialized zero for state inspection.
    #[wasm_bindgen(js_name = default)]
    pub fn default_value() -> Decimal {
        Self {
            inner: CoreDecimal::default(),
        }
    }

    /// Computes the exact product without changing either operand.
    pub fn mul_exact(&self, other: &Decimal) -> Result<Decimal, JsError> {
        self.inner
            .mul_exact(&other.inner)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }

    /// Rounds at a decimal-place precision after validating JS numeric arguments.
    pub fn rescale(&self, prec: f64, mode: f64) -> Result<Decimal, JsError> {
        let p = checked_i32(prec).map_err(JsError::new)?;
        let mode = checked_i32(mode).map_err(JsError::new)?;
        let mode = RoundingMode::from_raw(mode).map_err(library_error)?;
        self.inner
            .rescale(p as isize, mode)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }

    /// Compact decimal output from the Rust library.
    pub fn string(&self) -> String {
        self.inner.string()
    }

    /// Decimal output retaining the stored scale.
    pub fn string_with_trailing_zeros(&self) -> String {
        self.inner.string_with_trailing_zeros()
    }

    /// Complete exact representation for feasibility assertions.
    pub fn snapshot(&self) -> String {
        serde_json::json!({
            "coefficient": self.inner.coefficient().to_string(),
            "scale": self.inner.scale().to_string(),
            "initialized": !self.inner.is_nil()
        })
        .to_string()
    }
}

/// Build metadata; commit is injected by the reproducible build script.
#[wasm_bindgen]
pub fn metadata() -> String {
    serde_json::json!({
        "bridge_schema": 1,
        "commit": option_env!("DECIMAL_BUILD_COMMIT").unwrap_or("development"),
        "target": "wasm32-unknown-unknown",
        "features": [],
        "stage": "WEB-02 feasibility only"
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn numeric_parameters_cannot_wrap_or_truncate() {
        for bad in [f64::NAN, f64::INFINITY, 1.5, 2147483648.0, -2147483649.0] {
            assert!(checked_i32(bad).is_err());
        }
        assert_eq!(checked_i32(-1.0), Ok(-1));
        assert_eq!(checked_i32(2147483647.0), Ok(i32::MAX));
    }

    #[cfg(target_arch = "wasm32")]
    mod browser {
        use super::*;
        use wasm_bindgen_test::*;
        wasm_bindgen_test_configure!(run_in_browser);

        #[wasm_bindgen_test]
        fn exact_product_and_state_match_native_contract() {
            let a = Decimal::new_from_string("12.350").unwrap();
            let b = Decimal::new_from_string("3").unwrap();
            let result = a.mul_exact(&b).unwrap();
            assert_eq!(result.string(), "37.05");
            assert_eq!(result.string_with_trailing_zeros(), "37.050");
            assert_eq!(
                result.snapshot(),
                r#"{"coefficient":"37050","initialized":true,"scale":"3"}"#
            );
            assert_eq!(a.string_with_trailing_zeros(), "12.350");
        }

        #[wasm_bindgen_test]
        fn rounding_and_returned_errors_are_real_rust_calls() {
            let value = Decimal::new_from_string("1.245").unwrap();
            assert_eq!(value.rescale(2.0, 5.0).unwrap().string(), "1.24");
            assert_eq!(value.rescale(2.0, 3.0).unwrap().string(), "1.25");
            assert!(value.rescale(2.0, 6.0).is_err());
            assert!(value.rescale(-1.0, 5.0).is_err());
            assert!(Decimal::new_from_string("invalid").is_err());
            assert_ne!(
                Decimal::default_value().snapshot(),
                Decimal::new_from_string("0").unwrap().snapshot()
            );
        }
    }
}
