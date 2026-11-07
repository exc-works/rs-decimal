# Reading and maintaining the code

## Start with the usage contract

Read the executable examples and type documentation in `src/lib.rs`, then the rustdoc for individual methods. `Decimal` represents `coefficient × 10^-scale`; Default's uninitialized zero and NullDecimal's valid flag are independent states. Numerical equality does not imply identical representation, so tests should also check coefficient, scale, initialization, and valid where needed.

Public APIs retain Go-compatible naming; internal names should make units clear, such as coefficient, scale, and weight. Avoid comments that repeat self-explanatory statements. Explain rounding proofs, boundary-check order, protocol units, failure atomicity, and resource limits. Every public entry point needs documentation and runnable examples; return values, precision units, errors, and panics must match the implementation.

## Module responsibilities

| Module | Responsibilities |
| --- | --- |
| decimal / integer | Representation, parsing, comparison, and basic integer operations |
| arithmetic / rounding | Exact coefficient arithmetic, scale alignment, and the single set of signed rounding rules |
| conversion / formatting | Exact rational and IEEE conversions, text output |
| interval / transcendental | Outward-rounded intervals and certification loops with increasing precision |
| root / logarithm / exponential | Domain algorithms, argument reduction, root midpoint tests, and series remainder bounds |
| text / binary / json / yaml / xml / bson | Protocol boundaries and atomic updates of complete state |
| sql / postgres | Driver-neutral SQL semantics and SQLx NUMERIC base-10000 encoding |
| validation / validation::messages | Numerical rules and message rendering / separately stored language templates |
| config | Explicit input types and Serde field adapters that preserve raw JSON |

`raw()` only borrows the coefficient. Arithmetic alignment borrows coefficients with the same scale and allocates only for nonzero values that need shifting. This avoids repeated cloning and keeps the invariant that alignment preserves precision in one place. Parsing validates the full syntax before using a u64 path for up to 18 digits. Longer inputs still use arbitrary-precision integers; the fast path must not become an input-range limit.

## Acceptance criteria for numerical changes

Round interval endpoints downward/upward respectively; return success only when both endpoints round to the same value on the target decimal grid. Do not optimize by adding an empirical epsilon, using floating-point approximations, or treating NoConvergence as success. Root operations compare exact midpoint powers directly so floating-point error cannot affect ties-to-even.

For behavior changes, first add a regression that distinguishes the old behavior from the correct behavior, then run the relevant unit tests, full Go differential suite, and independent oracle. Performance changes should include before/after measurements with the same environment and inputs, distinguishing in-process measurements from end-to-end differential timings. CI runs MSRV/stable, paired 32/64-bit, cross-platform, and PostgreSQL checks. After changing a lower PR, propagate fixes to dependent PRs and review the final head again.
