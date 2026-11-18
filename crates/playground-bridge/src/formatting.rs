use super::{
    ByteBuffer, CoreDecimal, Decimal, library_error, limits, parameter_error, resource_error, tick,
};
use rs_decimal::GoFormat;
use wasm_bindgen::prelude::*;
fn scalar(text: &str) -> Result<char, &'static str> {
    let mut chars = text.chars();
    let value = chars.next().ok_or("expected one Unicode scalar value")?;
    if chars.next().is_some() {
        return Err("expected one Unicode scalar value");
    }
    Ok(value)
}
fn options(text: &str) -> Result<GoFormat, &'static str> {
    let value: serde_json::Value =
        serde_json::from_str(text).map_err(|_| "invalid format options JSON")?;
    let object = value
        .as_object()
        .ok_or("format options must be an object")?;
    let mut f = GoFormat::default();
    for (key, value) in object {
        match key.as_str() {
            "verb" => f.verb = scalar(value.as_str().ok_or("verb must be a string")?)?,
            "width" | "precision" => {
                let number = if value.is_null() {
                    None
                } else {
                    let n = value
                        .as_u64()
                        .ok_or("width/precision must be nonnegative integers or null")?;
                    if n > u32::MAX as u64 {
                        return Err("width/precision must fit wasm32 usize");
                    }
                    Some(n as usize)
                };
                if key == "width" {
                    f.width = number;
                } else {
                    f.precision = number;
                }
            }
            "left" | "plus" | "space" | "zero" | "alternate" => {
                let flag = value.as_bool().ok_or("format flags must be booleans")?;
                match key.as_str() {
                    "left" => f.left = flag,
                    "plus" => f.plus = flag,
                    "space" => f.space = flag,
                    "zero" => f.zero = flag,
                    _ => f.alternate = flag,
                }
            }
            _ => return Err("unknown format option"),
        }
    }
    Ok(f)
}
fn format_budget(value: &CoreDecimal, f: GoFormat) -> Result<(), &'static str> {
    if f.width.unwrap_or(0) > limits::BUFFER_BYTES {
        return Err("playground format width exceeds output budget");
    }
    if let Some(p) = f.precision {
        if matches!(f.verb, 'f' | 'F') && p > i32::MAX as usize {
            return Ok(());
        } // core rejects before allocation on wasm32
        if p > limits::SCALE {
            return Err("playground format precision exceeds budget");
        }
        if matches!(f.verb, 'f' | 'F') {
            limits::rescale(value, p as i32)?;
        }
    }
    Ok(())
}
fn append_budget(value: &CoreDecimal, length: usize) -> Result<(), &'static str> {
    // The coefficient is already bounded; this small temporary bounds both append variants.
    let digits = value
        .coefficient()
        .to_string()
        .trim_start_matches('-')
        .len();
    let text_bound = digits.max(value.scale() + 1) + 2;
    limits::buffer(length.saturating_add(text_bound))
}
#[wasm_bindgen]
impl Decimal {
    pub fn format_with_separators(
        &self,
        thousands: &str,
        decimal: &str,
    ) -> Result<String, JsValue> {
        tick()?;
        let thousands = scalar(thousands).map_err(parameter_error)?;
        let decimal = scalar(decimal).map_err(parameter_error)?;
        Ok(self.inner.format_with_separators(thousands, decimal))
    }
    pub fn format_go(&self, text: &str) -> Result<String, JsValue> {
        tick()?;
        if text.len() > limits::INPUT_BYTES {
            return Err(resource_error(
                "playground format options exceed input budget",
            ));
        }
        let f = options(text).map_err(parameter_error)?;
        format_budget(&self.inner, f).map_err(resource_error)?;
        self.inner.format_go(f).map_err(library_error)
    }
    pub fn append_text(&self, dst: &mut ByteBuffer) -> Result<(), JsValue> {
        tick()?;
        append_budget(&self.inner, dst.bytes.len()).map_err(resource_error)?;
        self.inner.append_text(&mut dst.bytes);
        Ok(())
    }
    pub fn append_text_with_trailing_zeros(&self, dst: &mut ByteBuffer) -> Result<(), JsValue> {
        tick()?;
        append_budget(&self.inner, dst.bytes.len()).map_err(resource_error)?;
        self.inner.append_text_with_trailing_zeros(&mut dst.bytes);
        Ok(())
    }
}
#[wasm_bindgen]
pub fn go_format_default() -> Result<String, JsValue> {
    tick()?;
    let f = GoFormat::default();
    Ok(serde_json::json!({"verb": f.verb.to_string(), "width": f.width, "precision": f.precision, "left": f.left, "plus": f.plus, "space": f.space, "zero": f.zero, "alternate": f.alternate}).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn format_options_are_strict_and_allocations_preflighted() {
        for text in [
            "[]",
            r#"{"width":-1}"#,
            r#"{"precision":0.5}"#,
            r#"{"plus":1}"#,
            r#"{"verb":"ff"}"#,
            r#"{"extra":true}"#,
        ] {
            assert!(options(text).is_err());
        }
        let d = CoreDecimal::new(1);
        assert!(format_budget(&d, options(r#"{"width":65537}"#).unwrap()).is_err());
        assert!(format_budget(&d, options(r#"{"precision":1001}"#).unwrap()).is_err());
        assert!(append_budget(&d, limits::BUFFER_BYTES).is_err());
        assert!(scalar("💠").is_ok());
        assert!(scalar("").is_err());
    }
}
