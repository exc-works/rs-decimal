use super::{
    CoreDecimal, Decimal, checked_i32, library_error, limits, parameter_error, resource_error, tick,
};
use rs_decimal::BigInt;
use wasm_bindgen::prelude::*;

fn precision(value: f64) -> Result<isize, JsValue> {
    let value = checked_i32(value).map_err(parameter_error)?;
    if value > limits::SCALE as i32 {
        return Err(resource_error(
            "playground constructor exceeds scale budget",
        ));
    }
    Ok(value as isize)
}
fn signed(value: &str) -> Result<i64, JsValue> {
    limits::input(value).map_err(resource_error)?;
    value
        .parse()
        .map_err(|_| parameter_error("value must fit i64"))
}
fn unsigned(value: &str) -> Result<u64, JsValue> {
    limits::input(value).map_err(resource_error)?;
    value
        .parse()
        .map_err(|_| parameter_error("value must fit u64"))
}
fn big(value: &str) -> Result<BigInt, JsValue> {
    limits::input(value).map_err(resource_error)?;
    value
        .parse()
        .map_err(|_| parameter_error("value must be an integer"))
}
fn wrap(value: Result<CoreDecimal, rs_decimal::DecimalError>) -> Result<Decimal, JsValue> {
    value.map(|inner| Decimal { inner }).map_err(library_error)
}

#[wasm_bindgen]
impl Decimal {
    pub fn new_from_int(value: f64) -> Result<Decimal, JsValue> {
        tick()?;
        Ok(Self {
            inner: CoreDecimal::new_from_int(checked_i32(value).map_err(parameter_error)? as isize),
        })
    }
    pub fn new_from_decimal(value: &Decimal) -> Result<Decimal, JsValue> {
        tick()?;
        Ok(Self {
            inner: CoreDecimal::new_from_decimal(&value.inner),
        })
    }
    pub fn new_with_prec(value: &str, prec: f64) -> Result<Decimal, JsValue> {
        tick()?;
        wrap(CoreDecimal::new_with_prec(signed(value)?, precision(prec)?))
    }
    pub fn new_from_int64(value: &str, prec: f64) -> Result<Decimal, JsValue> {
        tick()?;
        wrap(CoreDecimal::new_from_int64(
            signed(value)?,
            precision(prec)?,
        ))
    }
    pub fn new_from_uint64(value: &str, prec: f64) -> Result<Decimal, JsValue> {
        tick()?;
        wrap(CoreDecimal::new_from_uint64(
            unsigned(value)?,
            precision(prec)?,
        ))
    }
    pub fn new_from_big_int(value: &str) -> Result<Decimal, JsValue> {
        tick()?;
        Ok(Self {
            inner: CoreDecimal::new_from_big_int(&big(value)?),
        })
    }
    pub fn new_from_big_int_with_prec(value: &str, prec: f64) -> Result<Decimal, JsValue> {
        tick()?;
        wrap(CoreDecimal::new_from_big_int_with_prec(
            &big(value)?,
            precision(prec)?,
        ))
    }
    pub fn new_with_append_prec(value: &str, prec: f64) -> Result<Decimal, JsValue> {
        tick()?;
        wrap(CoreDecimal::new_with_append_prec(
            signed(value)?,
            precision(prec)?,
        ))
    }
    pub fn new_from_uint_with_append_prec(value: &str, prec: f64) -> Result<Decimal, JsValue> {
        tick()?;
        wrap(CoreDecimal::new_from_uint_with_append_prec(
            unsigned(value)?,
            precision(prec)?,
        ))
    }
}
