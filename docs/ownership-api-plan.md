# Decimal ownership and Rust API evolution

Historical implementation plan: requirements to preserve Go panic forms or defer fallible APIs are superseded by the [current Rust error policy](rust-api.md#recoverable-errors-and-migration). Numerical, state and resource contracts remain unchanged.

Status: proposed; implementation starts only after Codex review of this design PR reports no outstanding actionable findings. Base: `main` at `0dc7109e2c04f0e2c87d6df023e851845f973d7d` (remote verified on 2026-11-07). This is an additive evolution of the [technical design](technical-design.md), not a replacement of the pinned Go compatibility contract.

## Problem and decisions

`Decimal` owns `Option<BigInt>` and a scale. Its arithmetic borrows inputs and returns an independent result; decoding with `&mut self` can replace a value. Immutable arithmetic semantics do not require copying an exclusively owned temporary. With the locked num-bigint 0.4.6, cloning copies its digit Vec, whereas moving transfers ownership. Owned BigInt addition/subtraction can reuse storage, subject to sign, operand size, and growth.

Keep the representation and dependencies. First remove redundant copies, then offer explicit borrowing and consuming interfaces. Do not introduce Arc/Rc, interior mutability, unsafe code, a fixed-width coefficient, or a new rounding default. Arc would favor workloads dominated by shared large values, but adds allocation/reference-counting costs to arithmetic; small-coefficient storage is a separate representation project requiring measurements and full compatibility validation.

All existing public signatures, error categories, panic payloads/check order, numeric results, scale, initialization, and protocol bytes remain compatible. No old entry point is removed or deprecated in this series. New Rust-specific APIs are documented separately from the 166-entry Go inventory and do not inflate compatibility coverage.

## Four local optimizations (implementation PR A)

### A1. Return checked arithmetic temporaries directly

`safe_add` and `safe_sub` currently produce a result then call `must_non_negative(&self)`, cloning the result coefficient. Instead, bind the result, apply the same `assert!` with the same `Negative value` message, and move the result out. The public `must_non_negative` contract stays unchanged, including initialization of default zero.

Validate positive, zero, negative and default operands; check complete representation and the string panic payload. Neither input may change.

### A2. Borrow the unscaled division operand

In `quo_with_prec`, retain the existing order: validate precision, reject zero divisor, return initialized zero for zero numerator, check positive shift overflow. For `shift = p - self.scale + other.scale`, borrow both operands at zero, otherwise construct only the scaled numerator or denominator. Use scoped branches or Cow; do not clone the unchanged side. Keep the same `rounded_div` and its signed rounding algorithm.

Tests cover negative/zero/positive shift, both divisor signs, all modes, exact and inexact division, negative precision, zero divisor and native-maximum scale fast paths. No claim is made that the BigInt division implementation itself is allocation-free.

### A3. Consume owned alignment temporaries

Keep `coefficient_at_scale` borrowing equal-scale/zero coefficients. Match its Cow results instead of unconditionally converting both to references. Dispatch to the owned/reference BigInt addition/subtraction combinations where available. Never call `into_owned` on both borrowed operands. Subtraction must retain left/right order, even when only the right side owns its aligned coefficient.

Tests cover both scale orders, both signs, cancellation, initialized and default zero, long coefficients, output scale and input invariance. Allocation reuse is an opportunity, not a promise that carry/borrow growth needs no allocation.

### A4. Borrow magnitude for read-only formatting

Replace BigInt `abs().to_string()` uses with `magnitude().to_string()` where the magnitude is only read: fixed-point output, integer-format output, scientific significant-digit extraction, and significant-figures digit counting. Retain sign emission, trailing-zero handling, rounding and width behavior. Do not mechanically change `Decimal::abs`, arithmetic sign operations, or interval mathematics.

Validate signed/zero/large values through all affected formatting paths and existing Go formatting fixtures. Output bytes must be unchanged.

## Ownership-friendly APIs (implementation PR B)

Use a dedicated module where helpful; do not make the private unchecked representation constructor public.

| New entry | Contract |
| --- | --- |
| `coefficient(&self) -> &BigInt` | Borrow the unscaled coefficient; default zero borrows the existing shared zero. The reference is tied to self, cannot mutate the Decimal, and does not preserve initialization information independently. |
| `into_coefficient(self) -> BigInt` | Move out the coefficient; default becomes integer zero. Discards scale and initialization, explicitly documented. |
| `try_from_coefficient(coefficient: BigInt, scale: usize) -> Result<Self, DecimalError>` | Take ownership without cloning. Accept `0..=isize::MAX`; larger scale returns `Overflow`. Always initializes, including coefficient zero. Does not apply the text parser's scale cap or strip trailing zeros. |
| `scale(&self) -> usize` | Explicit nonnegative decimal-place count; complementary to legacy `precision() -> isize`. |
| `into_rescaled(self, prec: isize, mode: RoundingMode) -> Self` | Same checks, panics, numeric/scale/initialization result as `rescale`; consume self, reuse coefficient at unchanged scale, multiply owned coefficient when increasing scale, use the existing rounding primitive when decreasing. Default zero must become initialized even on the no-op branch. |
| `add_owned(self, other: &Self) -> Self` | Consume the left operand, borrow the right, exact addition at max scale; initialize the result. Reuse owned storage during alignment and addition where possible. |
| `sub_owned(self, other: &Self) -> Self` | Same ownership and scale rules as add_owned, preserving subtraction order. |
| `max_ref<'a>(&'a self, other: &'a Self) -> &'a Self` | Borrow the larger operand; on numeric equality return the first operand, preserving its representation. |
| `min_ref<'a>(&'a self, other: &'a Self) -> &'a Self` | Borrow the smaller operand with the same first-on-tie rule. |

The existing borrowed methods remain efficient borrowed paths: do not implement them by first cloning self and delegating wholesale to an owned method. Shared helpers may express alignment invariants without forcing clones. Existing copy constructors and `big_int()` keep their contracts; rustdoc points callers to the borrowing/moving alternatives.

`add_owned` and `sub_owned` deliberately make consumption explicit without requiring std::ops trait imports. Example flow: `a.add_owned(&b).sub_owned(&c).into_rescaled(2, mode)`. Borrowed operands remain usable; consumed operands follow normal Rust move rules. These APIs do not promise zero allocations for every sign/size combination.

Validate coefficient and full state against borrowed equivalents over a signed/scale matrix, including default zero, initialized scaled zero, cancellation and significant carry/borrow. Verify that borrowed coefficient access and max_ref/min_ref preserve identity using safe reference equality. Add rustdoc compile-fail examples for use after move and attempts to mutate a borrowed coefficient; no raw pointer/allocator implementation is required in library code.

## Rust style audit and disposition (implementation PR C)

| Existing surface | Issue | Decision in this series |
| --- | --- | --- |
| `new_from_int*`, `new_from_big_int` | Borrowed constructors impose copies; Go-style naming | Add `From` for owned BigInt and all Rust signed/unsigned integer primitives, including i128/u128 and pointer widths, at scale zero. Keep old constructors. No narrowing casts. |
| `new_from_string` / `must_from_string` | Fallible parsing versus panic constructor | Prefer existing `FromStr` / `text.parse::<Decimal>()` in Rust examples; retain compatibility constructors. |
| `new_from_decimal` | Redundant with Clone; move often suffices | Document move versus explicit Clone, without adding another alias or removing the old one. |
| `big_int`, `precision` | Copy hidden by getter spelling; precision actually means scale | Add the PR B accessors and explain costs/units. |
| `add/sub/neg` | Missing operator syntax | Implement Add/Sub only for `&Decimal` with `&Decimal` RHS, and Neg only for `&Decimal`, delegating to the compatible borrowed operations. |
| Owned `Add/Sub/Neg` traits | Imported by-value traits can win method resolution over existing inherent `&self` methods and unexpectedly consume `value.add(&rhs)` / `value.neg()` | Defer all such implementations on owned Decimal; use the explicit PR B consuming methods. Compile-test borrowed legacy calls with traits in scope and subsequent reuse of inputs. |
| `mul`, `quo` | Rounding parameter cannot be supplied by operator syntax; mul_exact has different scale rules | Do not implement Mul/Div, and do not assign an implicit rounding mode. |
| `cmp -> i32`, `equal`, `gt/gte/lt/lte` | Overlap with Ordering and comparison operators | Retain legacy methods; teach `Ord::cmp(&a, &b)` and `==`, `<`, etc. No second comparison alias. |
| `max/min -> Self` | Returning an owned copy for selection | Add PR B borrowed selection; explicitly retain first-on-tie representation (std extrema need not share this tie policy). |
| `safe_add/safe_sub`, `must_*` | “Safe” means nonnegative assertion, not Result-returning checked arithmetic | Document the assertion contract and allocation fix; do not silently rename or change failure shape. |
| `RoundingMode::from_raw(i32)` | Panics for invalid external discriminants | Add `TryFrom<i32, Error = DecimalError>` with `InvalidArgument` for values outside 0..=6. Preserve from_raw's exact string panic for old callers. |
| `int64/uint64 -> (T, bool)` | The bool reports exact, in-range integer conversion success | Document that fractions return (0, false), overflow fails, and uint64 additionally requires nonnegative input. Defer new integer conversion traits to a separately reviewed API proposal. |
| `float32/float64 -> (T, bool)` | Bool means exactness rather than success | Keep and document the distinction; no lossy From trait or discarded exactness flag. |
| `new_from_float*`, rational conversions | Nonfinite/nonterminating inputs and rounding policies need explicit failure rules | Defer broad conversion-trait additions; keep existing entry points. |
| `r#mod`, `quo_rem`, `remainder` | Go names can be confused with Rust remainder and fractional decomposition | Prefer existing modulo/quo_rem in examples and explain remainder's unscaled tuple. No Rem trait in this series. |
| `Default` and `NullDecimal { decimal, valid }` | Unusual unset zero and independently controlled validity | Preserve compatibility states; do not replace with initialized zero or Option<Decimal>. |
| Type-wide `#[must_use]`, AddAssign/SubAssign | Potential new downstream warnings or extra mutation surface | Defer; explicit owned operations address the present task without widening the mutation API. |
| Arithmetic panic APIs | More Rust-like fallible APIs would be useful | Defer a full try_* layer and its error-policy design. Never implement it with catch_unwind. New coefficient/mode conversion APIs above return Result directly. |
| `Display`/`string`, protocol unmarshal methods | Display already exists; decode methods intentionally mutate atomically | Prefer to_string/format! in examples, preserve canonical output and existing decoder atomicity. General formatter width/precision redesign is outside this series. |

PR C adds a Rust usage guide, links it from README, updates crate docs and records the additive entries outside the upstream inventory. Each new public entry/trait has docs and runnable examples. No additional dependencies or MSRV increase (1.85).

## Validation and performance evidence

Every code PR runs formatting, workspace all-target/all-feature Clippy, relevant contract tests and rustdoc tests, and debug/release tests. Run the existing core gates (all fixed Go differential fixtures, 512 seeded cases and independent oracle), plus the affected arithmetic/rounding oracle certificates and formatting fixtures. Rust-only APIs require direct tests because the Go inventory cannot validate them. CI remains responsible for actually executing MSRV, paired 32/64-bit, platform and PostgreSQL jobs; do not describe unobserved CI results as passed.

Performance comparisons use the same benchmark source and inputs before/after, sequential release runs, no competing compilation, and include allocation/destruction. Extend the core benchmark with safe arithmetic, positive/zero/negative division shifts, both mixed-scale add/sub orders, small/large coefficients, and affected formatting. For PR B compare owned pipelines to equivalent borrowed pipelines with equivalent input construction/cloning costs included; do not hide a clone inside only one side's setup. Coefficient borrow/move versus copy is a distinct workload from arithmetic.

Record environment, commit IDs, median/range and input sizes. Distinguish eliminated source-level clones from measured time reduction, retain regressions/noise, and make no universal speed claims. Preserve the original performance baseline report; new measurements get a separate report. Semantic tests do not assert allocator counts or pointer stability of BigInt's private digit buffers.

Required regressions include initialization state, first-on-tie selection, unchanged inputs, all rounding modes, signed division shifts, scale overflow/check ordering and identical formatting. Native-max tests must use fast paths rather than attempt astronomical allocations. Keep the fixed Go baseline unchanged.

## PR sequence and review gate

| Unit | Branch / base | Scope |
| --- | --- | --- |
| Design D | codex/decimal-ownership-plan / main | This proposal only; no library implementation before a clean Codex review. |
| A | codex/decimal-local-copies / D | Four local optimizations, their regression coverage, benchmarks and measured report. No new public API. |
| B | codex/decimal-owned-api / A | Ownership-friendly API table, direct state/move tests and pipeline measurements. |
| C | codex/decimal-rust-api / B | Conversion/borrowed operator traits, Rust usage guide and additive API inventory. |

Use stacked PR bases so each review diff contains one unit. Do not merge automatically. Each PR description links its dependency and records exact reviewed head and validation. Run Codex review after opening each PR, fix actionable findings, push fixes and review the updated head again. A clean result means an actual completed review with no outstanding actionable findings, not an exit code, a queued request, silence, or absent comments. Only the GitHub Codex connector (chatgpt-codex-connector[bot]), triggered with `@codex review`, satisfies this gate. Local Codex CLI reviews and self-review are not substitutes. If connector review is unavailable or incomplete, dependent work waits and the status is reported honestly.

If a lower branch changes, propagate it to dependent branches without losing work, rerun affected validation and review each changed final head again. Gate D before starting A/B/C; gate each implementation PR before starting its dependent unit. Tests and Codex review supplement each other. Numerical-contract changes and the deferred API/representation projects require separate proposals.
