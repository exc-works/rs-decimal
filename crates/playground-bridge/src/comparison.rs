use super::{CoreDecimal, Decimal, tick};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Decimal {
    pub fn precision(&self) -> Result<i32, JsValue> {
        tick()?;
        Ok(self.inner.precision() as i32)
    }
    pub fn sign(&self) -> Result<i32, JsValue> {
        tick()?;
        Ok(self.inner.sign())
    }
    pub fn is_zero(&self) -> Result<bool, JsValue> {
        tick()?;
        Ok(self.inner.is_zero())
    }
    pub fn is_not_zero(&self) -> Result<bool, JsValue> {
        tick()?;
        Ok(self.inner.is_not_zero())
    }
    pub fn is_positive(&self) -> Result<bool, JsValue> {
        tick()?;
        Ok(self.inner.is_positive())
    }
    pub fn is_negative(&self) -> Result<bool, JsValue> {
        tick()?;
        Ok(self.inner.is_negative())
    }
    pub fn big_int_text(&self) -> Result<String, JsValue> {
        tick()?;
        Ok(self.inner.big_int().to_string())
    }
    // Preserve the core inherent method name and its -1/0/1 result.
    #[allow(clippy::should_implement_trait)]
    pub fn cmp(&self, other: &Decimal) -> Result<i32, JsValue> {
        tick()?;
        Ok(self.inner.cmp(&other.inner))
    }
    pub fn equal(&self, other: &Decimal) -> Result<bool, JsValue> {
        tick()?;
        Ok(self.inner.equal(&other.inner))
    }
    pub fn not_equal(&self, other: &Decimal) -> Result<bool, JsValue> {
        tick()?;
        Ok(self.inner.not_equal(&other.inner))
    }
    pub fn gt(&self, other: &Decimal) -> Result<bool, JsValue> {
        tick()?;
        Ok(self.inner.gt(&other.inner))
    }
    pub fn gte(&self, other: &Decimal) -> Result<bool, JsValue> {
        tick()?;
        Ok(self.inner.gte(&other.inner))
    }
    pub fn lt(&self, other: &Decimal) -> Result<bool, JsValue> {
        tick()?;
        Ok(self.inner.lt(&other.inner))
    }
    pub fn lte(&self, other: &Decimal) -> Result<bool, JsValue> {
        tick()?;
        Ok(self.inner.lte(&other.inner))
    }
    pub fn abs(&self) -> Result<Decimal, JsValue> {
        tick()?;
        Ok(Self {
            inner: self.inner.abs(),
        })
    }
    pub fn neg(&self) -> Result<Decimal, JsValue> {
        tick()?;
        Ok(Self {
            inner: self.inner.neg(),
        })
    }
    pub fn max(&self, other: &Decimal) -> Result<Decimal, JsValue> {
        tick()?;
        Ok(Self {
            inner: CoreDecimal::max(&self.inner, &other.inner),
        })
    }
    pub fn min(&self, other: &Decimal) -> Result<Decimal, JsValue> {
        tick()?;
        Ok(Self {
            inner: CoreDecimal::min(&self.inner, &other.inner),
        })
    }
}
