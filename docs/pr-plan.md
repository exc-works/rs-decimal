# rs-decimal PR plan

Historical implementation plan: requirements to preserve Go panic forms or defer fallible APIs are superseded by the [current Rust error policy](rust-api.md#recoverable-errors-and-migration). Numerical, state and resource contracts remain unchanged.

The numbers below define the stacked PR plan, execution order, and dependencies. Update the public API compatibility matrix and test-coverage status as each PR merges. See the [implementation plan](implementation-plan.md) for overall goals and the [technical design](technical-design.md) for interface details.

## PR 01 Project setup and compatibility baseline

Dependencies: none. Suggested title: `chore: establish Rust crate and Go compatibility baseline`.

Deliver the Cargo skeleton, module directories, basic CI, MIT license and provenance notes, pinned Go SHA, source contracts, and fixture inventory. Export the public Go API inventory and record Rust entry points, return types, errors, scale, and owning PRs. Include easily missed items such as SafeAdd/SafeSub, AppendText, MostSignificantBit, and framework helpers. Explicitly map or defer items without a corresponding Rust interface.

Store the matrix in `docs/api-compatibility.md`, following the fields and status rules in the technical design's “API compatibility matrix” section. Export all public declarations, including conditional BSON entry points, through the Go AST. Check stable API IDs for missing, duplicate, and unknown entries; give each item a delivery tier, PR, and treatment. If concrete signatures and tests still need design, retain an explicit status rather than marking them compatible prematurely.

Acceptance: cargo check/test, fmt, and Clippy pass; SHA and file checksums match; the API inventory is complete; dependency compatibility and provisional MSRV are confirmed; both Git author and committer use the designated Skywalker Luke identity.

## PR 02 Differential execution infrastructure

Dependencies: 01. Suggested title: `test: add versioned Go and Rust conformance protocol`.

Deliver the JSONL protocol, Go reference program, Rust executor framework, driver scripts, complete-state comparator, panic/error classification, and regression storage. Pin the Go reference dependency to the SHA; make the development checkout path configurable so CI does not depend on a local ../decimal.

Also establish the separate `tools/go-oracle/` module and candidate-verification protocol, exercising the process with a minimal exact-rational case. Record extraction provenance as specified in the technical design and remove testing.T and production Decimal dependencies. Correct candidates must pass; incorrect coefficients, scales, or initialization states must fail; unsupported/uncertified cannot count as passes. Subsequent mathematical PRs add their certificates incrementally rather than deferring all independent verification to PR 14.

Acceptance: protocol schema and round-trip tests pass; deliberately introduced differences in value, scale, state, and error type are detected; timeouts and process failures never count as success. Unimplemented operations explicitly report unsupported instead of returning empty success placeholders or counting as compatible.

## PR 03 Types and construction

Dependencies: 02. Suggested title: `feat: port decimal representation parsing and comparison`.

Deliver Decimal and NullDecimal state structures, ErrorKind, basic constructors, exact string parsing, Clone, sign/state inspection, comparison, and Min/Max. Initial compact output is for validation only; complete formatting follows in PR 10a.

Acceptance: cover uninitialized/initialized zero, parsed-zero normalization, trailing zeros, scientific notation, invalid inputs, and parsing boundaries; equal values with different representations; Min/Max retaining the first item on equality; comparisons at large scale without unnecessary expansion; and accessor copies that do not expose internal mutable state.

## PR 04 Rounding and scale

Dependencies: 03. Suggested title: `feat: port exact rounding and scale operations`.

Deliver integer helpers, seven rounding modes, the Rescale family, Shift, Floor/Ceil/Truncate/Round, StripTrailingZeros, and SignificantFigures.

Acceptance: exact-rational oracle coverage for positive/negative midpoints, parity, direction, and Unnecessary; target scale and zero state preserved; invalid raw-mode conversions fail; scale overflow behaves identically in debug/release. Check that SignificantFigures does not pad trailing zeros.

## PR 05 Basic arithmetic and powers

Dependencies: 04. Suggested title: `feat: port arithmetic with single final rounding`.

Deliver Add/Sub/Raw/Safe, Mul/MulExact/MulWithPrec, Quo/QuoWithPrec, QuoRem/Mod, Power, integer parts, and remainder checks.

Acceptance: rational validation and randomized differential tests for all modes; double-rounding counterexamples; divisor signs; 0^0, negative powers, minimum exponent, and exact-intermediate boundaries; default scale; nonnegative assertions in Safe methods; and unchanged inputs. Include at least one regression for an extreme-scale fast path.

## PR 06 Integer, rational, and floating-point conversions

Dependencies: 05. Suggested title: `feat: preserve exact conversion and floating point boundaries`.

Deliver BigInt/BigRat, integer ranges and exact flags, shortest-decimal floating-point input, and correctly rounded f32/f64 output.

Acceptance: cover neighborhoods of 2^24/2^53, binary midpoints, subnormals, largest finite values, overflow, signed zero, and NaN/Infinity; single rounding to f32; random float bit patterns and an oracle for large-integer ratios; terminating/nonterminating rationals; and unchanged inputs. Valid conversions with exact=false must not be mistaken for failures.

## PR 07 Exact rounding of roots

Dependencies: 06. Suggested title: `feat: port certified square and integer roots`.

Deliver integer-root upper bounds, Newton iteration, exact midpoint comparisons, Sqrt/ApproxRoot and WithPrec variants, preserving budgets and error order.

Acceptance: independent rounding-cell certificates; negative odd/even roots; default Sqrt/ApproxRoot scale of `max(input scale, 20)`, each tested at input scales 0, 20, and 30 against WithPrec at the same target, comparing complete results. Separately test WithPrec at p=0, below input scale, and above input scale; high-degree roots and budget boundaries; 401st- and 500th-root regressions; and success for every root must-succeed case. Extract the Go root oracle to verify Rust candidates and reject altered results.

## PR 08 Interval primitives and Ziv driver

Dependencies: 04. Suggested title: `feat: add outward intervals and rounding certification`.

Deliver primitives for fractions, multiplication/division, integer multipliers, reciprocals, directed binary scaling, endpoint rounding, and guard growth. Keep the API internal without exposing incomplete mathematical functions.

Acceptance: exact-rational proof samples establish enclosure; negative and zero-crossing intervals; buffers do not corrupt inputs; forced precision retries; intervals crossing rounding boundaries must be rejected; and budget exhaustion cannot return approximate success.

## PR 09 Logarithms and exponential

Dependencies: 06, 08. Suggested title: `feat: port certified logarithms and exponential`.

Deliver Ln/Log2/Log10/Exp and all WithPrec variants, retaining exact fast paths, argument reduction, remainder bounds, and error/panic forms. If the diff is too large, split into 09a logarithms and 09b exponential, each independently validated.

Acceptance: the extracted rigorous rational-interval oracle verifies Rust candidates and rejects mutations. Default Ln/Log2/Log10/Exp scale is `max(input scale, 20)`; test each at input scales 0, 20, and 30 against WithPrec at the same target, comparing complete results. Separately test WithPrec at p=0, below/above input scale; values near 1, on either side of powers of 2 and rounding transitions, and large negative exponents; and multiple precisions. Once both PR 07 and this PR are complete, run all 45 fixed must-succeed cases and independent certificates. NoConvergence cannot substitute for promised success.

## PR 10a Formatting

Dependencies: 05. Suggested title: `feat: preserve decimal formatting behavior`.

Deliver compact and trailing-zero Decimal output, AppendText, separator formatting, the Go-compatible formatting entry point, and Display documentation.

Acceptance: width, sign, precision, #, f/e/E/g/G, and all other existing verbs; %g choosing notation after rounding carry; scientific notation without expanding zeros at large scale; AppendText behavior; and unchanged inputs for all formatting operations.

After PRs 01–09 and 10a are complete, run the core preview exit gates in the implementation plan, completing architecture, build-mode, oracle, packaging, and documentation checks within that scope. This checkpoint does not depend on 10b–13 and does not automatically publish the crate.

## PR 10b Text protocols and null values

Dependencies: 10a. Suggested title: `feat: preserve Text and Param decoding states`.

Deliver Decimal/NullDecimal Text/Param entry points and NullDecimal.String, explicitly documenting null rules and in-place update semantics.

Acceptance: distinguish empty text from whitespace; valid/invalid NullDecimal and internal uninitialized state; successful state transitions; failure atomicity; and encoding without input mutation. Complete formatting acceptance remains covered by 10a.

## PR 11 JSON, Serde, and null values

Dependencies: 06, 10b. Suggested title: `feat: add JSON compatibility and Serde adapters`.

Deliver raw JSON number parsing, string encoding, Decimal/NullDecimal in-place updates, corresponding Serde adapters, and documented differences. Clearly separate serde/json feature boundaries.

Acceptance: number tokens beyond binary floating-point precision retain every digit; null, empty strings, missing fields, and different initial receiver states; valid=true with an uninitialized inner value encodes as valid zero; all failures preserve state; and generic Deserialize and in-place entry points are tested separately.

## PR 12 Go binary interoperability

Dependencies: 04, 10a. Suggested title: `feat: support Go decimal binary encoding`.

Deliver precision-prefix and big.Int Gob payload encoding/decoding, Go-generated golden samples, and samples encoded by Rust for the reverse direction.

Acceptance: Go encode→Rust decode and Rust encode→Go decode; identical canonical encoding bytes; trailing-zero removal, uninitialized values, empty bytes, invalid versions, truncation, precision limits, and failure atomicity; plus 32-bit execution verifying conversion order.

## PR 13 YAML, XML, BSON, and generic scanning

Dependencies: 11. Suggested split: four separately reviewable PRs, 13a YAML, 13b XML, 13c BSON, and 13d Scan/Value.

Deliver raw-text/quote/tag semantics, XML elements and attributes, BSON String/Double/Int32/Int64/Decimal128/Null, a Scan input enum, and output rules. Generic scanning does not claim completion of any specific database driver.

Acceptance: reuse yaml_scalars; test octal, hexadecimal, and quoted numbers; XML whitespace; finite/special BSON Decimal128 values; null differences between entry points; failure atomicity everywhere; and compilation/tests with each feature enabled individually.

## PR 14 Release candidate validation

Dependencies: all of 01–13, including 10a and 10b. This is the protocol-compatible release candidate, not a prerequisite for the core preview. Suggested title: `test: enforce cross-language release compatibility gates`.

Deliver the complete API status matrix, cross-platform CI, extended fuzz-job configuration, independent oracle and regression reports, performance baselines with matching semantics, usage documentation, package-content checks, version declaration, and MSRV declaration.

Acceptance: all promised methods and must-succeed cases pass; no unexplained differential mismatches; debug/release and actually executed 32/64-bit pairings; default/individual/all-feature combinations; and cargo package validation in an isolated directory. Performance conclusions require checked results and measurement evidence. This PR does not automatically run cargo publish.

## PR 15 Subsequent ecosystem integrations

Dependencies: 14. Preparation may begin once the core stabilizes, but does not block the numerical core preview.

Choose adapters based on the databases and Web/configuration frameworks the user actually adopts, with a separate PR for each driver or framework. Prioritize one database implementation and exact numerical validation helpers, then extend configuration decoding and translated messages. Do not commit to every Rust framework at once.

Acceptance: live database round trips, NULL, precision, and out-of-range rejection tests; validation without floating-point comparisons; configuration decoding that preserves raw number text; and an explicit destination in the matrix for every legacy Go framework helper API. Future adapters must not change numerical-core semantics.

## Dependencies and review conventions

The main sequence is 01→02→03→04→05→06→07. Work on 08 may begin after 04; 09 depends on 06 and 08. Work on 10a may begin after 05; 10b depends on 10a. PR 11 depends on 06 and 10b, 12 on 04 and 10a, and 13 on 11. The core preview waits for 01–09 and 10a and runs independent gates. PR 14 waits for all in-scope implementation, including 10b, and gates protocol compatibility. These dependencies do not require parallel agents.

Each PR description states concrete behavior changes, corresponding Go APIs, input/output examples, validation evidence, and remaining differences. Do not combine algorithm refactoring, numerical-semantic changes, and performance optimization into a single hard-to-review PR. Every discovered mismatch must leave a regression; do not alter expected values to fit ported results.

PRs 01–15 are now fully implemented: 15a/15b/15c deliver PostgreSQL SQLx, exact validation/translation, and Serde JSON configuration respectively. All 166 API statuses must pass release checks. Each layer uses the preceding layer as its base. Final review and actual CI results are recorded in the corresponding PRs, and fixes propagate in dependency order. PRs are not merged and the crate is not published automatically.
