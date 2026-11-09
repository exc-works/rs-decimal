# rs-decimal technical design

The design preserves the [pinned baseline's behavioral contract](implementation-plan.md) while using Rust ownership and types to express data boundaries. Implement equivalent algorithms and state first, then optimize based on measurements.

## Project structure and dependencies

Start with one library crate and auxiliary test programs under tools. The original planned layout is shown below; these implementation files had not yet been created when the plan was written.

```text
src/
  lib.rs              Public exports
  decimal.rs          Types, state, construction, comparison
  rounding.rs         Seven rounding modes and quotient/remainder decisions
  arithmetic.rs       Addition, subtraction, multiplication, division, remainders, powers, scale
  integer.rs          Directed integer division, bit operations, powers of ten
  conversion.rs       Integer, rational, and floating-point conversions
  root.rs             Integer roots and exact midpoint certification
  interval.rs         Outward-rounded interval primitives
  transcendental.rs   Ziv driver
  logarithm.rs        Ln, Log2, Log10
  exponential.rs      Exp
  format.rs           Compact, trailing-zero, and Go-compatible formatting
  error.rs            Error categories and panic payloads
  nullable.rs         NullDecimal
  codec/              Text, JSON, Binary, YAML, XML, BSON
tests/               Contract, oracle, regression, and feature integration tests
testdata/            Pinned source data and differential regressions
tools/go-reference/  Reference program calling the pinned Go library
tools/go-oracle/     Independent mathematical verifier extracted from pinned Go tests
tools/conformance/   Rust executor and differential driver
```

Initial dependency candidates are num-bigint, num-integer, and num-traits, with num-rational for rational interfaces or tests. Select compatible versions, lock test dependencies, and record the minimum Rust version; the local Rust 1.99.1 installation does not automatically define the MSRV. The first release uses std and does not also promise no_std, C ABI, or dedicated wasm support.

Planned optional features are serde, json, yaml, xml, and bson. Serde provides type integration; json provides format adaptation that must preserve raw number text and state rules. They are separate compatibility layers. Select format libraries in their respective PRs after verifying raw-text preservation.

## Data representation

```rust
pub struct Decimal {
    coefficient: Option<BigInt>,
    scale: usize,
}

pub struct NullDecimal {
    pub decimal: Decimal,
    pub valid: bool,
}
```

Decimal fields are private. None means uninitialized and requires internal scale=0. Some(0) is initialized zero and may carry a nonzero scale. Default returns an uninitialized value. Numerical equality does not require identical representation: 1.0 equals 1.00. Tests must additionally compare scale and initialization state.

Internally, scale uses usize, but the public compatibility limit is isize::MAX, matching the nonnegative range of Go int at the same bit width. Compatibility interfaces accepting precision/shift use isize to preserve error paths for negative arguments. Intermediate scale arithmetic uses checked operations or wider integers. Entry points such as MulExact retain their own original limits rather than sharing the parser's limit.

The first release pairs Go/Rust at the same bit width. Fixtures mark native-integer boundaries that intentionally differ between 32-bit and 64-bit platforms. Binary encoding's u32 scale is platform-independent and checked before conversion to a native type.

Arithmetic accepts &self and &Decimal and returns new values. Provide Clone, not Copy. Big-integer accessors return copies. Cached powers of ten are read-only; operations exclusively own temporary buffers. Start with directly owned BigInt values, without introducing Arc or interior mutability in advance.

Implement PartialEq/Ord manually based on mathematical value, rather than deriving structural equality. If Hash is implemented, equal values with different scales must have the same hash. For numerically equal operands, Max/Min returns the first operand's representation, including uninitialized state. Express constants as read-only constants or constructors without copying Go's reassignable global-variable mechanism.

## API and errors

Public compatibility methods use corresponding snake_case names, such as mul_with_prec, quo_with_prec, power, and ln_with_prec. Preserve precision() as the counterpart of Precision, documenting that it means scale. SafeAdd/SafeSub require a nonnegative result and return InvalidArgument when it is negative.

Use an enum for the seven rounding modes. Safe Rust cannot construct an invalid enum; return InvalidArgument for invalid integer modes through the raw-value conversion entry point. Record these type-level differences in the API matrix.

Fallible internals and public APIs return Result for expected input, domain, precision, overflow and rounding failures. Only explicit must_* APIs retain their documented panic forms. Do not use catch_unwind for normal control flow. The differential executor reports Rust errors truthfully; its comparator normalizes only documented Go panic/category pairs and rejects unexpected Rust panics. See docs/rust-api.md for migration details.

ErrorKind maps the original sentinel categories. DecimalError must support matches(kind) or an equivalent query so decoding errors can match both Unmarshal and causes such as InvalidFormat, rather than retaining only the outermost category. Parser source errors may be preserved, but need not match Go's concrete types. Original non-sentinel input assertions map to InvalidArgument in ordinary Rust APIs; explicit must_* assertions still panic. Neither is mislabeled as numerical overflow.

Addition/subtraction operators may be added after the corresponding named methods stabilize. The first release does not implement Mul/Div traits without rounding parameters or introduce a default mode on its own. Optional try_* APIs may follow completion of the compatible core.

### API compatibility matrix

PR 01 creates `docs/api-compatibility.md`, exporting the initial inventory from Go declarations at the pinned commit. Do not manually construct the inventory solely from methods listed in this design. Include exported package functions, types, fields, constants, variables, and Decimal/NullDecimal methods, including entry points under the `bson` build tag; exclude tests and examples. Package functions and same-named methods, and Decimal and NullDecimal methods, receive separate entries.

Record the following fields for every item. Long rules may link to detailed entries in the same file:

| Field | Contents |
| --- | --- |
| Identity and source | Stable API ID, Go symbol, complete signature, file location at the pinned SHA, build conditions |
| Rust mapping | Target symbol, parameters, return types, and feature; unresolved parts explicitly marked design pending |
| Behavior | Value and scale, initialization/Valid state, input immutability or receiver-update rules |
| Failure | Result/panic/conversion-bool form, set of error categories, argument-check order |
| Delivery | Core/protocol/ecosystem tier, owning PR, direct port/platform mapping/explicit deferral and rationale |
| Validation | Contract case ID, differential operation/case_id, independent validation basis or reason it does not apply |
| Status | Not started, in progress, verified, or deferred; verified requires passing evidence |

Before PR 01 merges, each entry must specify its delivery tier, PR, and treatment. Concrete Rust signatures and test IDs may still be design pending at that stage, but must not be claimed compatible. Complete these fields and behavioral rules before merging the implementation PR. A platform mapping is a treatment, not automatic proof of validation. Deferred items must state scope and rationale and cannot count toward the verified proportion. At release, check every promised entry point for the relevant tier; aggregate library coverage cannot replace individual gates.

PR 01 extracts declarations through the Go AST and compares matrix ID sets, checking duplicates and unknown entries. Go signatures alone cannot establish panic, scale, or state behavior; review these fields against contracts, implementation, and tests. Later PRs update mappings and evidence. Do not remove source entries without explanation when the baseline is unchanged.

## Basic numerical algorithms

| Operation | Implementation rules |
| --- | --- |
| Add/Sub | Align to max(s,t), then compute exactly |
| AddRaw/SubRaw | Change the coefficient, preserving scale |
| Mul | Compute the exact coefficient product, then round directly to max(s,t) |
| MulExact | Use scale s+t, retaining the original intermediate-scale limit |
| MulWithPrec | Round directly to p; retain the fast path for tiny results that avoids huge powers of ten |
| QuoWithPrec | Apply exact decimal scaling to coefficient numerator/denominator, then one QuoRem and rounding step |
| Quo | Use the path targeting scale max(s,t) |
| QuoRem/Mod | Truncate the quotient toward zero; give the remainder the dividend's sign and preserve the specified scale |
| Power | Compute exact intermediate powers, then one final HalfEven rounding; preserve 0^0 and negative-exponent rules |
| Comparison | Check sign, equal scale, and magnitude bounds first; align exactly only when necessary |

Determine rounding from the exact remainder and divisor by comparing 2*abs(remainder) with abs(divisor). Do not compute a finite-precision approximation and then round again. At negative midpoints, HalfUp rounds away from zero and HalfDown toward zero.

The integer helper layer provides separate trunc_div_rem, floor_div, and ceil_div primitives. Go big.Int.Div uses Euclidean division and cannot be equated directly with floor_div for negative divisors. Prove positivity at each call site or adapt other cases explicitly. Also verify negative right shifts, bit lengths, and low-bit parity according to their mathematical use, rather than relying on similar names.

Initial budgets follow the original implementation: parsing precision magnitude is limited to 1<<17; roots allow at most 300 iterations; root degree, scale, and their products retain the original constraints; Ziv guard bits double from 64 to 8192. Transcendental working precision uses the same rational upper-bound conversion. These budgets are not a universal scale limit; exact fast paths and budget-check order must also match.

Wire scale and calculation digits use separately named internal constants even
though both currently equal 131072. No public resource context is required. See
the [resource boundary table](rust-api.md#resource-boundaries) for their different
units, exceptions and error categories. Certification checks room for its first
guard interval before allocating `10^p`. For nonzero integer `n` with bit length
`b`, `e >= b` proves `abs(n) / 10^e < 1/2`; rounding can therefore use `sign(n)/10`
without constructing the enormous denominator. This preserves directed rounding
and exactness errors as well as nearest rounding. It also applies to division
by a nonzero integer times `10^e`, retaining the divisor sign. These shortcuts
reduce unnecessary allocations but do not impose a memory or runtime budget.

## Roots and transcendental functions

Default output scale for Sqrt, ApproxRoot, Ln, Log2, Log10, and Exp is uniformly `max(input scale, 20)`, not a fixed 20 decimal places or 20 significant figures. WithPrec variants use exactly the specified nonnegative scale `p`, including `p < 20`; all successful results use HalfEven. This rule has been checked against the pinned baseline's `docs/numeric-contract.md`, `workingLogPrec` in `math.go`, and all six default entry points.

For every default entry point, test input scales 0, 20, and 30 and compare coefficient, scale, and initialization state with the corresponding WithPrec call at the default target scale. Separately test WithPrec at `p=0`, below input scale, and above input scale. Construct inputs explicitly from coefficient/scale so string parsing cannot normalize zeros. Retain the same state checks for representative exact values, zero results, and inexact results.

Roots first obtain an exact integer-root lower bound, then determine the final coefficient through exact midpoint powers. Preserve the original prefix upper bound and Newton iteration to minimize changes in the first release's success/failure domain. Only sqrt's underlying integer primitive may be replaced by a verified equivalent.

Intervals store integer lower/upper bounds sharing a 2^bits working scale. Every primitive maintains enclosure; multiplication, division, sign changes, squaring, and reciprocals must all verify endpoint direction.

Ln preserves binary reduction, reciprocal-direction selection, the atanh series, and remainder bounds. Log2/Log10 use ratios of enclosed logarithms and retain exact-power fast paths. Exp preserves argument reduction, Taylor remainders, repeated squaring, reciprocals for negative inputs, and certified zero for tiny results.

Ziv succeeds only when both interval endpoints have the same HalfEven result at the target scale. Budget exhaustion returns NoConvergence. Matching successive iterations or an epsilon cannot replace certification.

## Conversion and formatting

Parse floating-point inputs using shortest round-trip decimal semantics at the original bit width. Test the chosen spelling and scale when multiple shortest representations exist. Do not simply reuse Display if it produces different spelling or digits. Handle NaN/Infinity and negative zero according to the original entry point.

For floating-point output, locate the target exponent and significand directly from the exact integer ratio, then perform a single ties-to-even rounding using the integer remainder. Cover normal values, subnormals, rounding to zero, the largest finite value, and overflow. f32 must not pass through f64. Determine exact by mathematical equality, not matching round-trip text. Candidate dependencies providing equivalent semantics must pass the same gates.

Display produces the compact String form; explicit methods preserve trailing zeros. Provide a separate compatibility formatter for Go's f/e/E/g/G, width, sign, #, and related behavior. Do not assume Rust formatting syntax can express Go fmt literally. Include all existing public formatting capabilities in the inventory. %g must choose notation after rounding carry.

## Codecs and integrations

Every in-place decoder builds temporary state first and commits only on success. JSON null preserves the old state of plain Decimal but clears NullDecimal. Generic Deserialize constructs a new object with no previous state; document the distinction and provide in-place methods.

JSON numbers must be read from raw tokens without passing through f64. YAML must preserve quotes, tags, and raw number text while matching the baseline's octal, hexadecimal, and related rules. Adapt XML elements/attributes and BSON types/values separately; string round trips alone are insufficient.

Binary preserves the four-byte big-endian precision prefix and big.Int.GobEncode payload. Copy the corresponding Go implementation specification and golden samples; test positive/negative values, zero, invalid versions, truncation, excessive precision, and noncanonical inputs. Do not substitute generic Rust serialization. Strip trailing zeros before encoding. Empty bytes decode to initialized zero, so an uninitialized value's round trip does not preserve IsNil.

Scan/Value initially uses a format-neutral input enum for nil, integers, floats, text, and bytes, preserving original type-rejection rules. SQL drivers, validation, and configuration adapters follow in separate PRs with tests for their target frameworks. NullDecimal preserves the possibility of internal data when valid=false; do not automatically collapse it to Option<Decimal>.

## Differential protocol and independent validation

Use JSON Lines with one request/result per line. Fix schema_version, case_id, operation, parameters, and complete input states in the protocol. Encode coefficients, scales, and integer parameters as strings to avoid precision loss in JSON consumers. Encode floats as bit width plus raw hexadecimal bits, and bytes as hex. Represent uninitialized values with initialized=false, not decimal text alone.

Responses include outcome=ok/error/panic, complete result state, error_kinds, original inputs after the operation, and decoder receiver states before/after. The test driver classifies timeouts and process crashes as failures. stdout is reserved for protocol output; diagnostics and panic hooks write to stderr.

Fixtures include the source SHA, architecture, and generator version. The test driver feeds the same pregenerated inputs to both programs rather than depending on identical random generators in the two languages. Save a minimized regression for every failure; compare ordered sets of error categories independently of error strings.

The Go reference executor calls the pinned public API; the Rust executor calls the compatibility layer. Bound differential-test costs by bit length, scale, and operation count without narrowing the library contract. Run large fuzz jobs in isolated processes; elapsed-time limits must not be classified as NoConvergence.

The independent oracle uses exact rationals and rigorous bounds without calling production rounding, interval, or series helpers. Root and transcendental oracles in the pinned Go baseline are in `independent_oracle_test.go`. They depend on `testing.T`, package-internal Decimal state, and test rounding helpers, so they cannot be imported directly as a regular Go package.

Use source extraction: create a separate Go module in `tools/go-oracle/` and copy the required mathematical test logic and all helper dependencies, retaining licensing, source SHA, original file checksums, and change notes. Do not modify the upstream checkout or import the production decimal package. Replace `testing.T` failures with explicit verification results and Decimal parameters with ordinary data such as coefficient, scale, and initialized. Rational rounding helpers must also come from independent test logic, not production rounding. Review extraction differences separately; matching Go/Rust output alone does not establish independence.

The oracle receives `schema_version`, `case_id`, operation, complete inputs, rounding mode, and the Rust candidate result through a separate JSONL process, using the differential protocol's numerical encodings. It independently constructs exact rationals or rigorous intervals from inputs and checks candidate coefficient, target scale, and initialization state. It does not use the Go reference result as the mathematical expectation. Responses are `verified`, `mismatch`, `uncertified`, or `unsupported`, with reasoning or failure details. Only `verified` passes selected oracle cases; insufficient budget, unknown operations, timeouts, crashes, and protocol errors never pass. The contract/differential layers still verify complete input immutability and error categories. For nonnumerical interfaces, record why the oracle is inapplicable and what alternative evidence is used in the matrix.

PR 02 establishes the process protocol and a minimal executable exact-rational verifier, accepted using handwritten correct/incorrect candidates. PRs 04–06 add rounding, arithmetic, and conversion validation; PR 07 extracts root certificates; PR 09 extracts rigorous logarithm/exponential interval validation. At every stage, test rejection of altered coefficients, scales, and initialization states, retaining midpoint parity and certification-failure cases. After both PRs 09 and 07 are complete, send all 45 fixed must-succeed cases to the oracle. Additional default-scale regressions supplement the fixed corpus without modifying or removing it.

A Rust oracle may be added later but is not a prerequisite for independent validation in the first release. Existing 500/1500-digit reference texts provide cross-checks, not standalone certificates of correctly rounded final digits.

## CI and release

Each PR runs formatting, Clippy, unit, contract, and relevant differential tests. Critical numerical tests run in both debug and release. Release gates execute on supported 64-bit platforms and paired 32-bit environments; 32-bit tests must actually run, not merely cross-compile.

Check no-default, default, every individual feature, and all-feature combinations so all-features does not hide missing dependencies. Merge gates use fixed random seeds. Each numerical fuzzer should have at least a 20-second smoke test, with longer runs in scheduled jobs. Retain failing seeds as permanent regressions.

Benchmark parsing, small monetary values, multiplication/division at different bit lengths/scales, floats, roots, and high-precision mathematics separately, checking outputs first. Reports state Go/Rust versions, optimization settings, platform, and statistical method. Algorithmic equivalence does not imply a speed guarantee.

Before every public release, run cargo package and inspect package contents so fixtures and build dependencies for the Go reference/oracle do not affect consumer installation. Establish MSRV, version number, and crate-name availability before the first core preview release, then recheck at the protocol-compatible release candidate. See the implementation plan for specific gates. Publication is a separate action.

## Basic tooling references

- [num-bigint BigInt](https://docs.rs/num-bigint/latest/num_bigint/struct.BigInt.html): big integers, bit operations, and conversion primitives.
- [num-integer Integer](https://docs.rs/num-integer/latest/num_integer/trait.Integer.html): integer division in different directions.

These links are for capability checks. Pin dependency versions during implementation; latest documentation does not replace version-compatibility checks.
