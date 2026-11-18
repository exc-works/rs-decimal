//! Bounded browser bridge. Execution isolation is supplied by the runner.
mod arithmetic;
mod comparison;
mod constructors;
mod helpers;
mod limits;
mod ownership;
mod rounding;
use rs_decimal::{Decimal as CoreDecimal, DecimalError, RoundingMode};
use wasm_bindgen::prelude::*;

/// Browser wrapper retaining the original decimal value and representation.
#[wasm_bindgen]
pub struct Decimal {
    inner: CoreDecimal,
}

fn exception(category: &str, message: &str, kinds: &[String]) -> JsValue {
    let error = js_sys::Error::new(message);
    error.set_name(if category == "library" {
        "DecimalError"
    } else {
        "BridgeError"
    });
    js_sys::Reflect::set(
        &error,
        &JsValue::from_str("category"),
        &JsValue::from_str(category),
    )
    .expect("fresh error object");
    let values: js_sys::Array = kinds.iter().map(|v| JsValue::from_str(v)).collect();
    js_sys::Reflect::set(&error, &JsValue::from_str("kinds"), &values).expect("fresh error object");
    error.into()
}

fn library_error(error: DecimalError) -> JsValue {
    exception(
        "library",
        &error.to_string(),
        &error
            .kinds()
            .iter()
            .map(|k| format!("{k:?}"))
            .collect::<Vec<_>>(),
    )
}

fn resource_error(message: &str) -> JsValue {
    exception("resource", message, &[])
}

fn tick() -> Result<(), JsValue> {
    limits::tick().map_err(resource_error)
}

fn parameter_error(message: &str) -> JsValue {
    exception("parameter", message, &[])
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
    pub fn new_from_string(text: &str) -> Result<Decimal, JsValue> {
        tick()?;
        limits::input(text).map_err(resource_error)?;
        CoreDecimal::new_from_string(text)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }

    /// Internal exact signed-integer transport; the public JS facade accepts bigint.
    #[wasm_bindgen(js_name = new_i64_text)]
    pub fn new_i64_text(value: &str) -> Result<Decimal, JsValue> {
        tick()?;
        limits::input(value).map_err(resource_error)?;
        let value = value
            .parse::<i64>()
            .map_err(|_| parameter_error("value must fit i64"))?;
        Ok(Self {
            inner: CoreDecimal::new(value),
        })
    }

    /// Clones representation and initialization state.
    #[wasm_bindgen(js_name = clone)]
    pub fn clone_value(&self) -> Result<Decimal, JsValue> {
        tick()?;
        Ok(Self {
            inner: self.inner.clone(),
        })
    }

    /// Borrows the coefficient as an immutable JS bigint snapshot.
    pub fn coefficient_text(&self) -> Result<String, JsValue> {
        tick()?;
        Ok(self.inner.coefficient().to_string())
    }

    /// Reads the number of decimal places.
    pub fn scale(&self) -> Result<u32, JsValue> {
        tick()?;
        Ok(self.inner.scale() as u32)
    }

    /// Tests uninitialized state, independently of numerical equality.
    pub fn is_nil(&self) -> Result<bool, JsValue> {
        tick()?;
        Ok(self.inner.is_nil())
    }

    /// Consumes this wrapper, including when the library returns an error.
    pub fn into_rescaled(self, prec: f64, mode: f64) -> Result<Decimal, JsValue> {
        tick()?;
        let p = checked_i32(prec).map_err(parameter_error)?;
        let mode = RoundingMode::from_raw(checked_i32(mode).map_err(parameter_error)?)
            .map_err(library_error)?;
        limits::rescale(&self.inner, p).map_err(resource_error)?;
        self.inner
            .into_rescaled(p as isize, mode)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }

    /// Creates uninitialized zero for state inspection.
    #[wasm_bindgen(js_name = default)]
    pub fn default_value() -> Result<Decimal, JsValue> {
        tick()?;
        Ok(Self {
            inner: CoreDecimal::default(),
        })
    }

    /// Computes the exact product without changing either operand.
    pub fn mul_exact(&self, other: &Decimal) -> Result<Decimal, JsValue> {
        tick()?;
        limits::product(&self.inner, &other.inner).map_err(resource_error)?;
        self.inner
            .mul_exact(&other.inner)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }

    /// Rounds at a decimal-place precision after validating JS numeric arguments.
    pub fn rescale(&self, prec: f64, mode: f64) -> Result<Decimal, JsValue> {
        tick()?;
        let p = checked_i32(prec).map_err(parameter_error)?;
        let mode = checked_i32(mode).map_err(parameter_error)?;
        let mode = RoundingMode::from_raw(mode).map_err(library_error)?;
        limits::rescale(&self.inner, p).map_err(resource_error)?;
        self.inner
            .rescale(p as isize, mode)
            .map(|inner| Self { inner })
            .map_err(library_error)
    }

    /// Compact decimal output from the Rust library.
    pub fn string(&self) -> Result<String, JsValue> {
        tick()?;
        Ok(self.inner.string())
    }

    /// Decimal output retaining the stored scale.
    pub fn string_with_trailing_zeros(&self) -> Result<String, JsValue> {
        tick()?;
        Ok(self.inner.string_with_trailing_zeros())
    }

    /// Complete exact representation for feasibility assertions.
    pub fn snapshot(&self) -> Result<String, JsValue> {
        tick()?;
        Ok(serde_json::json!({
            "coefficient": self.inner.coefficient().to_string(),
            "scale": self.inner.scale().to_string(),
            "initialized": !self.inner.is_nil()
        })
        .to_string())
    }
}

/// Nullable value with explicit cloned inner-value access.
#[wasm_bindgen]
pub struct NullDecimal {
    inner: rs_decimal::NullDecimal,
}

#[wasm_bindgen]
impl NullDecimal {
    /// Moves a decimal into a present nullable value, including uninitialized zero.
    pub fn new(decimal: Decimal) -> Result<NullDecimal, JsValue> {
        tick()?;
        Ok(Self {
            inner: rs_decimal::NullDecimal::new(decimal.inner),
        })
    }
    /// Creates an absent nullable value.
    #[wasm_bindgen(js_name = default)]
    pub fn default_value() -> Result<NullDecimal, JsValue> {
        tick()?;
        Ok(Self {
            inner: rs_decimal::NullDecimal::default(),
        })
    }
    /// Reads independent validity state.
    #[wasm_bindgen(getter)]
    pub fn valid(&self) -> Result<bool, JsValue> {
        tick()?;
        Ok(self.inner.valid)
    }
    /// Changes validity without clearing the stored decimal.
    #[wasm_bindgen(setter)]
    pub fn set_valid(&mut self, value: bool) -> Result<(), JsValue> {
        tick()?;
        self.inner.valid = value;
        Ok(())
    }
    /// Explicit adapter: copies the inner value; not a live mutable reference.
    pub fn get_decimal_clone(&self) -> Result<Decimal, JsValue> {
        tick()?;
        Ok(Decimal {
            inner: self.inner.decimal.clone(),
        })
    }
    /// Explicit adapter: replaces the inner value by moving the argument.
    pub fn set_decimal(&mut self, value: Decimal) -> Result<(), JsValue> {
        tick()?;
        self.inner.decimal = value.inner;
        Ok(())
    }
    /// Complete state snapshot, including hidden value when invalid.
    pub fn snapshot(&self) -> Result<String, JsValue> {
        tick()?;
        Ok(serde_json::json!({"valid":self.inner.valid,"decimal":{
            "coefficient":self.inner.decimal.coefficient().to_string(),
            "scale":self.inner.decimal.scale().to_string(),"initialized":!self.inner.decimal.is_nil()
        }}).to_string())
    }
}

/// Mutable byte-buffer adapter for future append_text mappings.
#[wasm_bindgen]
pub struct ByteBuffer {
    bytes: Vec<u8>,
}

#[wasm_bindgen]
impl ByteBuffer {
    /// Copies an input byte slice into an owned mutable buffer.
    pub fn new(bytes: &[u8]) -> Result<ByteBuffer, JsValue> {
        tick()?;
        limits::buffer(bytes.len()).map_err(resource_error)?;
        Ok(Self {
            bytes: bytes.to_vec(),
        })
    }
    /// Copies bytes out without exposing a view invalidated by Wasm growth.
    pub fn to_uint8_array(&self) -> Result<Vec<u8>, JsValue> {
        tick()?;
        Ok(self.bytes.clone())
    }
}

/// Original Go compatibility revision, not the web build commit.
#[wasm_bindgen]
pub fn compatibility_revision() -> Result<String, JsValue> {
    tick()?;
    Ok(rs_decimal::COMPATIBILITY_REVISION.to_owned())
}

/// Build metadata; commit is injected by the reproducible build script.
#[wasm_bindgen]
pub fn metadata() -> Result<String, JsValue> {
    tick()?;
    Ok(serde_json::json!({
        "bridge_schema": 1,
        "commit": option_env!("DECIMAL_BUILD_COMMIT").unwrap_or("development"),
        "target": "wasm32-unknown-unknown",
        "features": [],
        "stage": "WEB-04 resource policy",
        "limits": {"input_bytes": limits::INPUT_BYTES,"input_digits": limits::INPUT_DIGITS,"coefficient_bits": limits::COEFFICIENT_BITS,"scale": limits::SCALE,"buffer_bytes": limits::BUFFER_BYTES,"operations": limits::OPERATIONS}
    })
    .to_string())
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
            assert_eq!(result.string().unwrap(), "37.05");
            assert_eq!(result.string_with_trailing_zeros().unwrap(), "37.050");
            assert_eq!(
                result.snapshot().unwrap(),
                r#"{"coefficient":"37050","initialized":true,"scale":"3"}"#
            );
            assert_eq!(a.string_with_trailing_zeros().unwrap(), "12.350");
        }

        #[wasm_bindgen_test]
        fn resource_limits_run_inside_the_browser_wasm() {
            assert!(Decimal::new_from_string("1e100000").is_err());
            assert!(Decimal::new_from_string("1e-100000").is_err());
            let value = Decimal::new_from_string("1e-600").unwrap();
            assert!(value.mul_exact(&value).is_err());
            assert!(value.rescale(1001.0, 5.0).is_err());
            assert_eq!(value.scale().unwrap(), 600);
            let bounded = Decimal::new_from_string("1e-1000").unwrap();
            assert_eq!(bounded.string_with_trailing_zeros().unwrap().len(), 1002);
        }

        #[wasm_bindgen_test]
        fn rounding_and_returned_errors_are_real_rust_calls() {
            let value = Decimal::new_from_string("1.245").unwrap();
            assert_eq!(value.rescale(2.0, 5.0).unwrap().string().unwrap(), "1.24");
            assert_eq!(value.rescale(2.0, 3.0).unwrap().string().unwrap(), "1.25");
            assert!(value.rescale(2.0, 6.0).is_err());
            assert!(value.rescale(-1.0, 5.0).is_err());
            assert!(Decimal::new_from_string("invalid").is_err());
            assert_ne!(
                Decimal::default_value().unwrap().snapshot().unwrap(),
                Decimal::new_from_string("0").unwrap().snapshot().unwrap()
            );
        }
    }
}
