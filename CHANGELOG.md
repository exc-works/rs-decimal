# Changelog

## 0.1.0-alpha.1 — unreleased

- Avoid enormous powers of ten when rounding tiny values, multiplying at reduced
  precision, dividing by a sufficiently scaled denominator, or computing zero
  quotient/remainder. Preserve rounding, scale, initialization and error rules.
  Reject exhausted certification budgets before allocating the output multiplier;
  skip scale expansion for zero parsing, constructors and rational conversion.
  Document distinct wire/algorithm
  limits without adding a context API or narrowing supported numeric ranges.

- Breaking alpha API change: ordinary fallible constructors, arithmetic, precision
  changes, Log2, raw rounding-mode conversion, most-significant-bit validation and
  Go formatting return `Result` for expected failures. Negative precision also
  returns `Err` from APIs that already returned `Result`. Only explicit `must_*`
  methods retain input-related panics. Numerical/state/protocol behavior and
  existing resource limits are unchanged; see the Rust API migration guide.
- Update callers, error/state regressions and the differential comparator to
  verify the intentional Go-panic/Rust-error mapping without accepting Rust panics.

- Port the numerical and state contract of exc-works/decimal v0.8.0 at
  `a8791208e2d15483cf652cd2814b74b0c1a84187` (166 mapped API declarations).
- Preserve decimal scale, uninitialized zero, seven rounding modes and exact
  integer/rational conversions; certify root/logarithm/exponential rounding.
- Support text, Go binary and driver-neutral SQL conversion, with optional
  JSON, Serde, YAML, XML, BSON, SQLx PostgreSQL, validation and configuration.
- Supply fixed cross-language fixtures, an independent rational/interval oracle,
  32/64-bit paired checks, platform CI and real PostgreSQL round-trip tests.
- Document precision, null, error and panic contracts with executable examples.

This is an alpha API. Rust-specific mappings and intentional compatibility
boundaries are recorded in `docs/api-compatibility.md`. No crate has been
published by this change; publication requires a separately selected release commit.
