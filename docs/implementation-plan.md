# rs-decimal implementation plan

Historical implementation plan: requirements to preserve Go panic forms or defer fallible APIs are superseded by the [current Rust error policy](rust-api.md#recoverable-errors-and-migration). Numerical, state and resource contracts remain unchanged.

The goal is to preserve the Go library's numerical results, scale, initialization state, error categories, and protocol behavior in Rust. The implementation directly ports the existing algorithms using big integers, with shared cases, differential tests, and independent mathematical validation as delivery evidence.

## Compatibility baseline

- Source repository: `https://github.com/exc-works/decimal`.
- Pinned commit: `a8791208e2d15483cf652cd2814b74b0c1a84187`.
- Primary specification: [numeric-contract.md](https://github.com/exc-works/decimal/blob/a8791208e2d15483cf652cd2814b74b0c1a84187/docs/numeric-contract.md).
- Validation specification: [correctness-gates.md](https://github.com/exc-works/decimal/blob/a8791208e2d15483cf652cd2814b74b0c1a84187/docs/correctness-gates.md).
- The pinned baseline includes 45 must-succeed cases and fixtures for roots, transcendental functions, YAML scalars, and more.

Copy the contracts, required fixtures, and original MIT license into this repository, recording the source commit and file checksums. CI must use the pinned commit for the Go reference program; it must not follow main and silently change behavior.

When the contract, Go implementation, and independent oracle disagree, record a minimal reproducer and make a compatibility decision for each case. Neither assume Go output is automatically correct nor change compatible behavior without explanation.

## Scope and release tiers

| Tier | Deliverables | Public claims |
| --- | --- | --- |
| Core preview | Construction, state, comparison, rounding, arithmetic, conversion, roots, logarithms, exponential, formatting | Compatibility only for verified numerical APIs |
| Protocol-compatible release | Adds NullDecimal, Text, Param, JSON, Serde adapters, Binary, YAML, XML, BSON, and the Scan abstraction | Covered protocol entry points and Serde differences listed in the compatibility matrix |
| Ecosystem adapters | One priority database driver, validation, and configuration adapters | Individual mappings to Go frameworks and their differences |

Go's Gin, validator, mapstructure, and database/sql interfaces cannot be copied directly. Preserve portable input rules and state transitions through Rust entry points or adapters. Diagnostic wording, concrete Go runtime types, memory addresses, allocation counts, and thread scheduling are not requirements for literal compatibility.

The first release preserves nonnegative scale, max input scale for default Mul/Quo, and input scale with HalfEven for Power. Do not introduce negative scale, exact multiplication by default, new rounding modes, or automatic precision increases into the compatibility implementation.

## Implementation principles

1. The numerical core depends directly on big integers, without using bigdecimal or rust_decimal arithmetic as an intermediate layer.
2. The initial port preserves the original mathematical algorithms, fast paths, and computational budgets. Performance optimizations must pass the same compatibility gates.
3. Start with paired 64-bit Go and Rust validation; add paired 32-bit execution before release.
4. Each PR includes implementation, API mappings, and independently justified tests. Do not defer validation to the final PR.
5. Rust names use snake_case, with parameters and behavior corresponding to Go. Do not add multiplication/division operators with implicit rounding before the port is complete.
6. Framework integrations and format dependencies use optional features, so core users do not need Web or database dependencies.

## Milestones

| Milestone | PRs | Exit criteria |
| --- | --- | --- |
| M0 Baseline and test infrastructure | 01–02 | Buildable skeleton, pinned source version, two-program test protocol, and API inventory |
| M1 Decimal core | 03–06 | Exact and rounded arithmetic, state, conversions, and floating-point boundaries pass differential tests and the oracle |
| M2 Certified mathematics | 07–09 | Roots and all logarithm/exponential methods complete; all must-succeed cases succeed |
| Core preview checkpoint | 01–09, 10a | Core numerics and formatting complete, with the independent release gates below passing |
| M3 Protocol compatibility | 10b, 11–13 | Null behavior, failure atomicity, Binary interoperability, and all format entry points pass |
| M4 Release candidate | 14 | Debug/release, architecture, feature-matrix, and regression gates pass |
| M5 Ecosystem extensions | 15 onward | Selected drivers and frameworks have live integration tests and separate compatibility notes |

PR numbers describe logical units and may be split further based on actual diff size. See the [PR plan](pr-plan.md) for dependencies and acceptance criteria. Estimated dates do not replace correctness gates; update the schedule after M1 based on actual review and failure-resolution costs.

### Independent exit gates for the core preview

The core preview depends on PRs 01–09 and 10a, not on Text/Param, JSON/Serde, Binary, YAML, XML, BSON, Scan, or specific framework adapters. PR 10 is explicitly split into 10a for formatting and 10b for text protocols so protocol work does not block the core. Establishing the NullDecimal state structure early does not commit the preview to its full protocol capabilities.

- All core entry points in the API matrix are verified, each with a Rust mapping, state/error rules, and test evidence. Protocol and ecosystem entry points are explicitly marked undelivered.
- All six default mathematical methods verify `max(input scale, 20)` with input scales of 0, 20, and 30. Target scale for WithPrec is verified independently.
- All 45 fixed must-succeed cases succeed and pass independent certificates. Core contract, differential, and supplemental regression tests have no unexplained failures; unsupported operations, failed certification, and timeouts do not pass.
- fmt, Clippy, and core unit/contract/differential checks pass. Critical numerical tests actually execute in debug/release and paired 32/64-bit environments. Default, no-default, and feature combinations included in the preview pass; undelivered format features do not block the preview.
- Core numerical fuzz smoke tests and regressions for known failing seeds pass. A core performance baseline checks results before measuring, without claiming unmeasured speedups.
- Usage documentation, MSRV, version policy, licensing, and provenance are complete. `cargo package` and builds/tests in an isolated directory pass. Confirm package-name availability before public release and clearly label preview scope without claiming full protocol compatibility.

Run these gates at the core preview checkpoint, rather than deferring them to PR 14. PR 14 remains the complete release-candidate gate for protocol compatibility, rerunning core checks and adding every protocol and feature check. Passing checks does not trigger publication automatically.

## Main risks

| Risk | Mitigation |
| --- | --- |
| Floating-point conversions introduce double rounding or an incorrect exact flag | Round directly to the target binary format; compare raw bit patterns and exact rationals |
| Changes in signed division direction invalidate intervals | Encapsulate truncating, floor, and ceiling primitives; independently verify all sign combinations |
| Both implementations copy the same bug | Retain rational and rigorous interval oracles alongside differential tests |
| Errors or timeouts hide difficult inputs | Must-succeed cases must succeed; count successes, errors, panics, and timeouts separately |
| Serde loses raw number tokens or null-update semantics | Provide format-specific raw decoding and in-place compatibility methods; document generic Serde limitations |
| Extreme scale exhausts resources | Initially preserve each entry point's existing budget; bound test input costs without silently narrowing the public supported range |
| Documentation claims full compatibility while missing edge APIs | Export the inventory from the pinned Go commit and link every item to a Rust entry point, PR, and acceptance cases |

## Completion criteria

- Every public Go entry point has an explicit treatment and validation status in the compatibility matrix. All promised entry points within the release scope are verified, platform mappings explain differences, and deferred items explain why. No unexplained omissions are allowed.
- Fixed cases and executable independent certificates pass; identical inputs produce matching coefficients, scales, states, and error categories.
- Do not remove original must-succeed cases, lower their precision, or replace final-digit verification with tolerances.
- Encoding/decoding failures preserve the previous state; serialization, comparison, and arithmetic leave inputs unchanged.
- CI executes debug/release and supported architectures, checking default, no-default, individual-feature, and all-feature combinations.
- There are no unexplained differential mismatches. Timeouts, process exits, and oracle certification failures count as test failures, never passes.
- Measure performance with identical semantics and inputs; do not promise in advance that Rust will be faster.
- The README states implementation scope, minimum Rust version, features, source commit, and version policy. The Rust crate has its own version number and does not present itself as a Go release.

This plan has entered full implementation with user authorization. Code and stacked PRs are delivered according to docs/pr-plan.md; release gates and individual platform mappings are recorded in the API matrix. This work does not automatically merge PRs or publish the crate.
