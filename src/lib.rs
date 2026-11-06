//! Rust port of the exc-works/decimal numerical contract.
//!
//! This initial crate establishes the compatibility baseline. Numerical APIs
//! are introduced in subsequent, independently validated changes.
#![forbid(unsafe_code)]

/// Immutable Go source revision used by the compatibility suite.
pub const COMPATIBILITY_REVISION: &str = "a8791208e2d15483cf652cd2814b74b0c1a84187";
