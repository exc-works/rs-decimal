use std::{error::Error, fmt};

/// Stable categories shared by returned errors and typed panic payloads.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    /// A divisor is zero.
    DivideByZero,
    /// A parameter or validation rule is invalid.
    InvalidArgument,
    /// Input is not a supported finite decimal representation.
    InvalidFormat,
    /// A logarithm argument is nonpositive.
    InvalidLog,
    /// The requested precision is invalid.
    InvalidPrecision,
    /// The root degree is nonpositive.
    InvalidRoot,
    /// An even root was requested for a negative value.
    NegativeRoot,
    /// The bounded algorithm could not certify the rounded result.
    NoConvergence,
    /// A coefficient, scale, or operation exceeds its supported range.
    Overflow,
    /// Exact rounding was requested but nonzero digits would be lost.
    RoundUnnecessary,
    /// Decoding failed; another category may describe the underlying cause.
    Unmarshal,
}

/// An error may match multiple categories (e.g. Unmarshal and InvalidFormat).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecimalError {
    kinds: Vec<ErrorKind>,
    message: String,
}

impl DecimalError {
    /// Creates an error with one category and a human-readable message.
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kinds: vec![kind],
            message: message.into(),
        }
    }
    /// Tests category membership; an error can match more than one category.
    pub fn matches(&self, kind: ErrorKind) -> bool {
        self.kinds.contains(&kind)
    }
    /// Returns the sorted, deduplicated categories attached to this error.
    pub fn kinds(&self) -> &[ErrorKind] {
        &self.kinds
    }
    /// Adds a category without removing existing categories.
    pub fn with_kind(mut self, kind: ErrorKind) -> Self {
        if !self.matches(kind) {
            self.kinds.push(kind);
            self.kinds.sort();
        }
        self
    }
}
impl fmt::Display for DecimalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}
impl Error for DecimalError {}

pub(crate) fn error(kind: ErrorKind) -> DecimalError {
    DecimalError::new(kind, format!("decimal: {kind:?}"))
}
