//! Rust port of the exc-works/decimal numerical contract.
//!
//! Representation preserves scale and distinguishes uninitialized zero.
#![forbid(unsafe_code)]

/// Immutable Go source revision used by the compatibility suite.
pub const COMPATIBILITY_REVISION: &str = "a8791208e2d15483cf652cd2814b74b0c1a84187";

mod decimal;
mod error;
mod integer;
pub use decimal::{Decimal, NullDecimal, between, hundred, max, min, one, ten, zero};
pub use error::{DecimalError, ErrorKind};
pub use num_bigint::BigInt;
pub use num_rational::BigRational;
