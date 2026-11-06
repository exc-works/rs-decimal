use std::{error::Error, fmt};

/// Stable categories shared by returned errors and typed panic payloads.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    DivideByZero,
    InvalidArgument,
    InvalidFormat,
    InvalidLog,
    InvalidPrecision,
    InvalidRoot,
    NegativeRoot,
    NoConvergence,
    Overflow,
    RoundUnnecessary,
    Unmarshal,
}

/// An error may match multiple categories (e.g. Unmarshal and InvalidFormat).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecimalError {
    kinds: Vec<ErrorKind>,
    message: String,
}

impl DecimalError {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kinds: vec![kind],
            message: message.into(),
        }
    }
    pub fn matches(&self, kind: ErrorKind) -> bool {
        self.kinds.contains(&kind)
    }
    pub fn kinds(&self) -> &[ErrorKind] {
        &self.kinds
    }
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

pub(crate) fn fail(kind: ErrorKind) -> ! {
    std::panic::panic_any(DecimalError::new(kind, format!("decimal: {kind:?}")))
}
