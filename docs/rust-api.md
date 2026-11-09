# Rust API and ownership guide

The existing method names preserve value, scale and initialization contracts. Ordinary fallible APIs now return `Result` for expected failures, intentionally replacing Go panic behavior. This breaking alpha change supersedes the error-policy restrictions in the historical [ownership/API design](ownership-api-plan.md).

## Parse and construct exact values

```rust
use rs_decimal::{BigInt, Decimal};
let amount: Decimal = "12.350".parse()?;
let count = Decimal::from(3u32);
let large = Decimal::from(BigInt::from(u128::MAX));
let scaled = Decimal::try_from_coefficient(BigInt::from(12350), 3)?;
assert_eq!(amount, scaled);
assert_eq!(amount.scale(), 3);
# Ok::<(), rs_decimal::DecimalError>(())
```

`From` accepts all signed and unsigned Rust integer primitives and owned BigInt without narrowing, producing initialized scale-zero values. `try_from_coefficient` takes ownership and accepts a usize scale up to isize::MAX; larger scale returns Overflow. It retains scale even for zero and preserves trailing zeros and is not restricted by the text parser's scale-magnitude cap of 131072.

Prefer `parse` for fallible text input. `must_from_string` retains its panic behavior. Floating-point constructors now return `InvalidFormat` for nonfinite inputs. Rational conversion remains explicit about exactness and rounding.

## Resource boundaries

Ordinary calls need no context or limit configuration. Existing bounds have
different purposes; 131072 is not a universal limit on `Decimal` values.

| Boundary | Limit and failure |
| --- | --- |
| Text parsing | The exponent must fit `i32`; the absolute final scale must be at most 131072, including zero inputs. Failure is `InvalidFormat`. Integer digit count and input byte length are not capped. |
| Go binary | Encoded scale is at most 131072. Encoding checks after removing trailing zeros and returns `InvalidPrecision`; decoding checks before reading the coefficient and returns `Unmarshal`. |
| Direct coefficient constructors | Nonnegative scale through `isize::MAX`; no text-parser cap. A small coefficient with a large scale is cheap to store, but subsequent operations may be expensive. |
| General roots | Input scale, output scale, degree, and `degree * max(1, output scale)` must be at most 131072. Newton iteration is capped at 300. Exhaustion returns `NoConvergence`. Degree-one, zero and one fast paths run before these checks. |
| Logarithms / exponentials | Input and output scales are capped at 131072 before identity fast paths. General certification also needs room for guard bits and, for exponentials, estimated integer digits. Guard bits double from 64 to 8192 within the working-size budget; exhaustion returns `NoConvergence`. |
| Arithmetic | `mul_exact` caps summed scale at `i32::MAX`; a nonzero `shift` caps nonnegative result scale at `i32::MAX`. Division rejects a positive scaling shift above `isize::MAX`. General integer powers bound exponent times the larger of normalized coefficient bits and scale by `i32::MAX`. These are operation-specific bounds, not memory quotas. |
| Formatting and exact rational conversion | Fixed-point output expands stored scale; formatting width/precision can add padding. `big_rat()` may construct a large power of ten. There is no common output-byte or allocation cap. |

Shrinking a tiny value no longer constructs `10^n` when coefficient bit length
already proves a magnitude below half an output unit. Rescaling, owned rescaling,
rounded multiplication, and division with a sufficiently large denominator shift
use this proof while retaining all rounding modes, including `RoundUnnecessary`.
Floor/ceiling, zero-dividend remainder operations, zero append-precision
constructors and zero-to-rational conversion also avoid needless scale expansion.
Inputs remain unchanged and result scale/initialization are preserved.

For external inputs, bound request bytes **before parsing**, then validate the
accepted scale and coefficient size before computation. Bound requested output
precision/width and operation count in the application as well. Input length
alone is insufficient: a short scientific-notation string can expand into a much
larger value. Passing the checks above does not guarantee a memory or runtime
budget, and `Result` does not make arbitrary allocation failures recoverable.

## Choose borrowing, copying or consuming

| Need | Entry | Ownership and cost |
| --- | --- | --- |
| Inspect coefficient | `d.coefficient()` | Shared reference, no coefficient copy; lifetime tied to d |
| Obtain independent coefficient | `d.big_int()` | Copies coefficient digits |
| Take coefficient from a finished value | `d.into_coefficient()` | Consumes d and moves digits; discards scale and initialization; default yields integer zero |
| Preserve an independent Decimal | `d.clone()` | Copies coefficient digits and preserves scale/initialization |
| Transfer a Decimal | `let next = d;` | Moves ownership; no deep coefficient copy |
| Read decimal places | `d.scale()` | usize decimal places; legacy precision() returns isize |
| Select an existing value | `a.max_ref(&b)`, `a.min_ref(&b)` | Borrow the selected operand; first operand wins numeric ties |

A coefficient reference does not separately encode the distinction between unset zero and initialized zero. Read is_nil() when that distinction matters. No public mutable coefficient reference is exposed.

```rust
use rs_decimal::Decimal;
let a = Decimal::new_with_prec(100, 2).unwrap();
let b = Decimal::from(1);
assert_eq!(a, b);
assert!(std::ptr::eq(a.max_ref(&b), &a));
assert_eq!(a.max_ref(&b).scale(), 2);
```

## Borrow inputs, consume temporary results

```rust
use rs_decimal::{Decimal, RoundingMode};
let a: Decimal = "12.350".parse()?;
let b: Decimal = "0.650".parse()?;
let total = &a + &b;
assert_eq!(total.string_with_trailing_zeros(), "13.000");
assert_eq!((&a - &b).to_string(), "11.7");
assert_eq!((-&a).to_string(), "-12.35");
let result = total.sub_owned(&b).into_rescaled(2, RoundingMode::HalfEven)?;
assert_eq!(result.string_with_trailing_zeros(), "12.35");
assert_eq!(a.to_string(), "12.35");
# Ok::<(), rs_decimal::DecimalError>(())
```

`add_owned(self, &rhs)` and `sub_owned(self, &rhs)` consume only the left operand and return an initialized result at max scale. `into_rescaled(self, prec, mode)` consumes the value with the same numerical rules and returned errors as rescale. Its unchanged-scale path still initializes default zero. These methods can reuse buffers, but growth, alignment and rounding may allocate.

Operator implementations are deliberately limited to `&Decimal + &Decimal`, `&Decimal - &Decimal`, and `-&Decimal`. Implementing the corresponding traits on owned Decimal could change legacy dot-method resolution when std::ops traits are imported. Explicit consuming methods avoid that source-compatibility hazard. There is no Mul/Div operator or default rounding policy: use mul_exact, mul(..., mode), or quo_with_prec(..., mode) as appropriate. AddAssign/SubAssign remain outside the current API.

## Prefer standard Rust spelling where semantics already match

| Compatibility entry | Rust usage / semantic note |
| --- | --- |
| new_from_int* at scale zero | `Decimal::from(integer)` |
| new_from_decimal | Move when the old binding is finished; otherwise Clone |
| new_from_string | `text.parse::<Decimal>()` |
| string | `d.to_string()` or `format!("{d}")` for canonical text; string_with_trailing_zeros retains stored scale |
| equal/not_equal/gt/gte/lt/lte | `==`, `!=`, `>`, `>=`, `<`, `<=` |
| cmp returning -1/0/1 | `Ord::cmp(&a, &b)` returns Ordering; the inherent cmp remains compatible |
| RoundingMode::from_raw | Both `RoundingMode::try_from(raw)` and `from_raw(raw)` return InvalidArgument for values outside 0..=6 |
| safe_add/safe_sub/must_non_negative | safe_add/safe_sub return InvalidArgument for a negative result; only must_non_negative asserts and panics |
| int64/uint64 | Success requires an exact, in-range integer; fractions return (0, false), and uint64 additionally rejects negatives |
| float32/float64 | The bool reports exactness of the floating-point value, not the integer-conversion success contract |
| r#mod / remainder | Prefer modulo for the signed arithmetic remainder; remainder() instead returns the integer part and unscaled fractional coefficient |

The scale of numerically equal values may differ. Default is uninitialized numerical zero, while normal arithmetic initializes results. NullDecimal.valid alone governs nullness, so its inner value may remain populated when invalid. Decoder methods may replace mutable receivers on success and retain the documented failure/null state rules. Neither an Option-based representation change nor a breaking rename/deprecation is part of this series.

## Recoverable errors and migration

Ordinary fallible APIs return `Result<T, DecimalError>` directly. This is a breaking
change to the unpublished alpha API, not an additional `try_*` layer. Add `?` to
propagate errors, or explicitly match `Err`. Use `unwrap`/`expect` only when the
caller deliberately wants a panic. No implementation uses `catch_unwind` to turn
ordinary library failures into errors.

| Family | Current failure contract |
| --- | --- |
| Constructors with an explicit signed precision | `InvalidPrecision` for negative precision |
| `new_from_float32/64` | `InvalidFormat` for NaN or infinity |
| `rescale`, `into_rescaled`, `rescale_down`, and `*_with_prec` rounding | `InvalidPrecision`; selectable exact rounding may return `RoundUnnecessary` |
| `mul`, `mul_exact`, `mul_with_prec` | Forbidden rounding or existing scale overflow checks return errors |
| `quo`, `quo_down`, `quo_with_prec`, `quo_rem`, `modulo`, `r#mod` | Zero divisor and applicable precision/overflow/rounding failures return errors |
| `power`, `shift` | Existing divide-by-zero and overflow failures return errors |
| `safe_add`, `safe_sub` | `InvalidArgument` when the result is negative |
| `significant_figures` | `InvalidArgument` for nonpositive figures; `RoundUnnecessary` for forbidden rounding |
| Roots, logarithms, exponentials, and `new_from_big_rat_with_prec` | Previously returned errors stay errors; negative precision and forbidden rounding no longer panic; `log2` now returns Result too |
| `RoundingMode::from_raw` / `TryFrom<i32>`, `most_significant_bit` | Invalid mode / negative integer returns `InvalidArgument` |
| `format_go` | Fixed-point precision outside the native signed range returns `InvalidPrecision`; successful text and Go error-marker strings are unchanged |

```rust
use rs_decimal::{Decimal, ErrorKind, RoundingMode};
let price = Decimal::new_with_prec(123, 2)?;
let total = price.mul_exact(&Decimal::from(3))?;
let rounded = total.rescale(1, RoundingMode::HalfEven)?;
assert_eq!(rounded.to_string(), "3.7");
assert!(price.quo_with_prec(&Decimal::new(0), 2, RoundingMode::HalfEven)
    .unwrap_err().matches(ErrorKind::DivideByZero));
assert!(price.sqrt_with_prec(-1).unwrap_err().matches(ErrorKind::InvalidPrecision));
# Ok::<(), rs_decimal::DecimalError>(())
```

`new`, integer `From`, add/subtract, borrowed operators, `mul_down`, and argument-free
round/floor/ceil/truncate still return values directly because their fixed
parameters cannot produce these semantic failures. `must_from_string` and
`must_non_negative` retain their explicit panic behavior and payloads.

Check order, successful coefficient/scale/initialization, and borrowed inputs are
preserved. `into_rescaled` consumes the input even when it returns an error.
Existing resource limits and protocol/null/serialization rules are unchanged.
Internal invariant defects and allocation failure are outside the recoverable
error contract; returning Result does not promise a bounded memory or time cost.

The Go differential driver normalizes only documented Go panic cases to their
Rust error counterparts; an unexpected Rust panic still fails. Direct regressions
in `tests/errors.rs` also exercise invalid parameters, check order, input state,
and explicit must-method panics.

## Additive API inventory and evidence

These are Rust extensions, separate from the [166 upstream API entries](api-compatibility.md). They do not add to Go compatibility coverage.

| Entry / family | Direct evidence |
| --- | --- |
| coefficient, into_coefficient, try_from_coefficient, scale | tests/ownership.rs construction/range/state checks; rustdoc borrowing and compile-fail move examples |
| add_owned, sub_owned | tests/ownership.rs full-state equivalence over signs/scales/large coefficients and extreme zero fast paths |
| into_rescaled | tests/ownership.rs all seven modes, initialized/default zero, native limit and failure equivalence |
| max_ref, min_ref | tests/ownership.rs identity, first-on-tie scale/state and ordering tests |
| From<BigInt>, From<i8/i16/i32/i64/i128/isize/u8/u16/u32/u64/u128/usize> | tests/rust_api.rs each integer type's MIN/MAX and zero/one; arbitrary-size signed coefficient and initialized zero; rustdoc examples |
| Add<&Decimal> for &Decimal, Sub<&Decimal> for &Decimal, Neg for &Decimal | tests/rust_api.rs full-state comparison, operator/UFCS calls and legacy dot-call reuse with imported traits |
| TryFrom<i32> for RoundingMode | tests/rust_api.rs complete 0..=6 mapping, invalid extremes and returned InvalidArgument errors |

Measured workloads and their limits are in [local copy measurements](performance-local-copies.md) and [owned pipeline measurements](performance-owned-api.md). Borrowing a reference and creating an independent value are different contracts; choose the interface according to the caller's ownership needs.
