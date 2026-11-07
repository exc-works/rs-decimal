# Changelog

## 0.1.0-alpha.1 — unreleased

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
