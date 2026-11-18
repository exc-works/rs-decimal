//! Playground-only preflight. The core crate keeps its full numerical domain.
use rs_decimal::Decimal;
use std::cell::Cell;

pub const INPUT_BYTES: usize = 8192;
pub const INPUT_DIGITS: usize = 4096;
pub const COEFFICIENT_BITS: u64 = 16384;
pub const SCALE: usize = 1000;
pub const BUFFER_BYTES: usize = 65536;
pub const OPERATIONS: u32 = 10000;
thread_local! { static CALLS: Cell<u32> = const { Cell::new(0) }; }

pub fn tick() -> Result<(), &'static str> {
    CALLS.with(|calls| {
        let count = calls.get();
        if count >= OPERATIONS {
            Err("playground operation budget exhausted")
        } else {
            calls.set(count + 1);
            Ok(())
        }
    })
}

pub fn input(text: &str) -> Result<(), &'static str> {
    if text.len() > INPUT_BYTES {
        return Err("playground input exceeds 8192 UTF-8 bytes");
    }
    let text = text.trim();
    let (mantissa, exponent) = match text.find(['e', 'E']) {
        Some(i) => match text[i + 1..].parse::<i32>() {
            Ok(exponent) => (&text[..i], exponent as i64),
            // Malformed/overflowing exponents are rejected by the core before allocation.
            Err(_) => return Ok(()),
        },
        None => (text, 0),
    };
    let digits = mantissa.bytes().filter(u8::is_ascii_digit).count();
    let fraction = mantissa.split_once('.').map_or(0, |(_, f)| f.len());
    let scale = fraction as i64 - exponent;
    let expanded = digits as i64 + (-scale).max(0);
    if scale.unsigned_abs() > SCALE as u64 || expanded > INPUT_DIGITS as i64 {
        return Err("playground input exceeds scale or coefficient budget");
    }
    Ok(())
}

pub fn rescale(value: &Decimal, precision: i32) -> Result<(), &'static str> {
    // Negative precisions retain the core's InvalidPrecision behavior.
    if precision < 0 {
        return Ok(());
    }
    let scale = precision as usize;
    let growth = scale.saturating_sub(value.scale()) as u64 * 4;
    if scale > SCALE || value.coefficient().bits() + growth > COEFFICIENT_BITS {
        return Err("playground rescale exceeds scale or coefficient budget");
    }
    Ok(())
}

pub fn product(left: &Decimal, right: &Decimal) -> Result<(), &'static str> {
    if left.scale() + right.scale() > SCALE
        || left.coefficient().bits() + right.coefficient().bits() > COEFFICIENT_BITS
    {
        return Err("playground product exceeds scale or coefficient budget");
    }
    Ok(())
}

pub fn buffer(length: usize) -> Result<(), &'static str> {
    if length > BUFFER_BYTES {
        Err("playground buffer exceeds 65536 bytes")
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exponent_expansion_and_scale_are_checked_before_parsing() {
        for text in ["1e100000", "1e-100000", "0e-100000"] {
            assert!(input(text).is_err());
        }
        assert!(input(&"9".repeat(INPUT_DIGITS + 1)).is_err());
        assert!(input(&format!("{}e1000", "9".repeat(INPUT_DIGITS))).is_err());
        assert!(input("1e1000").is_ok());
        assert!(input("1e-1000").is_ok());
        assert!(input("invalid").is_ok()); // core owns syntax errors
    }
    #[test]
    fn chained_operations_cannot_escape_value_bounds() {
        let scaled = Decimal::new_from_string("1e-600").unwrap();
        assert!(product(&scaled, &scaled).is_err());
        let large = Decimal::new_from_string(&"9".repeat(3000)).unwrap();
        assert!(product(&large, &large).is_err());
        assert!(rescale(&large, 1000).is_ok());
        assert!(rescale(&large, 1001).is_err());
        assert!(buffer(BUFFER_BYTES + 1).is_err());
    }
    #[test]
    fn exhausted_budget_stays_exhausted() {
        CALLS.with(|calls| calls.set(OPERATIONS - 1));
        assert!(tick().is_ok());
        assert!(tick().is_err());
        assert!(tick().is_err());
        CALLS.with(|calls| calls.set(0));
    }
}
