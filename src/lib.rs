//! Exact decimal arithmetic with explicit scale and Go-compatible rounding.
//!
//! A [`Decimal`] stores an arbitrary-size integer coefficient and a nonnegative
//! scale: `coefficient * 10^-scale`. Precision parameters mean **decimal places**,
//! not significant digits. Borrowed arithmetic leaves inputs intact; explicit
//! consuming methods can reuse owned storage. Decoders may replace mutable receivers.
//!
//! # Start with exact input
//!
//! ```
//! use rs_decimal::{Decimal, RoundingMode};
//! let price: Decimal = "12.350".parse()?;
//! let total = price.mul_exact(&Decimal::new(3))?;
//! assert_eq!(total.string(), "37.05");
//! assert_eq!(total.string_with_trailing_zeros(), "37.050");
//! let third = Decimal::new(1).quo_with_prec(&Decimal::new(3), 2, RoundingMode::HalfEven)?;
//! assert_eq!(third.string_with_trailing_zeros(), "0.33");
//! # Ok::<(), rs_decimal::DecimalError>(())
//! ```
//!
//! # Borrow values or consume temporaries
//!
//! ```
//! use rs_decimal::{Decimal, RoundingMode};
//! let a: Decimal = "1.25".parse()?;
//! let b = Decimal::from(2u32);
//! let sum = &a + &b;
//! assert_eq!(sum.to_string(), "3.25");
//! assert_eq!(a.scale(), 2);
//! let result = sum.sub_owned(&b).into_rescaled(1, RoundingMode::HalfEven)?;
//! assert_eq!(result.to_string(), "1.2");
//! let mode = RoundingMode::try_from(5)?;
//! assert_eq!(mode, RoundingMode::HalfEven);
//! # Ok::<(), rs_decimal::DecimalError>(())
//! ```
//!
//! [`Decimal::coefficient`] borrows digits; [`Decimal::big_int`] copies them;
//! [`Decimal::into_coefficient`] consumes the Decimal and discards scale/state.
//! Integer `From` conversions initialize scale zero. `+`, `-` and unary `-`
//! support borrowed operands; use [`Decimal::add_owned`] / [`Decimal::sub_owned`]
//! to consume temporaries. Multiplication and division keep explicit rounding.
//!
//! # Representation and null
//!
//! Numeric equality ignores scale: `1.0 == 1.00`. [`Decimal::default`] is an
//! uninitialized numerical zero; [`Decimal::is_nil`] distinguishes it from
//! initialized zero. [`NullDecimal::valid`] alone determines SQL/config nullness.
//! Ordinary arithmetic initializes its result, while cloning preserves state.
//!
//! ```
//! use rs_decimal::{Decimal, NullDecimal};
//! let unset = Decimal::default();
//! assert!(unset.is_nil());
//! assert_eq!(unset, Decimal::new(0));
//! let present_zero = NullDecimal::new(unset);
//! assert!(present_zero.valid);
//! assert!(!NullDecimal::default().valid);
//! ```
//!
//! # Rounding and failure contracts
//!
//! [`Decimal::mul`] rounds to the larger input scale; [`Decimal::mul_exact`]
//! keeps the sum of the scales. Explicit `*_with_prec` methods select the result
//! scale. [`RoundingMode::Down`] means toward zero, **not** toward negative infinity.
//!
//! Fallible constructors, arithmetic, precision changes, roots, logarithms and
//! exponentials return [`Result`]. Invalid precision, zero divisors, overflow and
//! forbidden rounding are returned as [`DecimalError`], including through `?`.
//! Only explicitly named `must_*` methods panic for expected input failures.
//! Infallible operations such as addition, subtraction and scale-zero rounding
//! still return values directly. Internal invariant bugs and allocation failures
//! are outside this recoverable-error contract.
//!
//! Coefficients are arbitrary-size, not resource-unlimited. Parsing bounds scale
//! magnitude to 131072; certified transcendental algorithms have finite budgets
//! and return [`ErrorKind::NoConvergence`] rather than an uncertified approximation.
//! Application limits on input length, scale, formatting width and operation count
//! remain necessary for untrusted workloads; formatting may allocate proportional
//! to scale and width.
//!
//! # Optional integrations
//!
//! No feature is enabled by default. Features are additive:
//!
//! | Feature | Entry points and exactness |
//! | --- | --- |
//! | `serde` | String/integer/null serialization; rejects floating-point deserialization |
//! | `json` | Original JSON number tokens via `unmarshal_json` |
//! | `config` | JSON-specific Serde field adapters in `config` (enables `serde` and `json`) |
//! | `validation` | Exact rules and localized messages in `validation` |
//! | `postgres` | SQLx 0.8.6 NUMERIC and NUMERIC[] traits; application chooses runtime/TLS |
//! | `yaml` / `xml` | Scalar/element decoding; encoding returns content for an emitter |
//! | `bson` | BSON type-byte and raw-payload conversion |
//!
//! Text, Go binary and driver-neutral [`ScanValue`] support are always available.
//! JSON null preserves an existing `Decimal`, but clears a `NullDecimal`.
//! SQL NULL resets a `Decimal` to uninitialized zero. Decoder errors preserve
//! the receiver's coefficient, scale and validity.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Immutable Go source revision used by the compatibility suite.
pub const COMPATIBILITY_REVISION: &str = "a8791208e2d15483cf652cd2814b74b0c1a84187";

mod arithmetic;
mod binary;
pub use binary::PRECISION_FIXED_SIZE;
mod conversion;
mod decimal;
mod error;
mod exponential;
mod formatting;
mod integer;
mod ownership;
pub use formatting::GoFormat;
mod interval;
#[cfg(feature = "json")]
mod json;
mod logarithm;
mod root;
mod rounding;
mod rust_api;
#[cfg(feature = "serde")]
mod serde_impl;
mod text;
mod transcendental;
pub use conversion::most_significant_bit;
pub use decimal::{Decimal, NullDecimal, between, hundred, max, min, one, ten, zero};
pub use error::{DecimalError, ErrorKind};
pub use num_bigint::BigInt;
pub use num_rational::BigRational;
pub use rounding::RoundingMode;

#[cfg(feature = "yaml")]
mod yaml;

#[cfg(feature = "xml")]
mod xml;

#[cfg(feature = "bson")]
mod bson;

mod sql;
pub use sql::ScanValue;

#[cfg(feature = "postgres")]
mod postgres;

#[cfg(feature = "validation")]
pub mod validation;

#[cfg(feature = "config")]
pub mod config;
