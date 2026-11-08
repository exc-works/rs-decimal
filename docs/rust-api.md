# Rust API and ownership guide

The Go-compatible methods remain available with their existing value, scale, initialization and error contracts. Rust applications can use the additional entry points below without changing representation. The approved scope and deferred alternatives are in the [ownership/API design](ownership-api-plan.md).

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

Prefer `parse` for fallible text input. `must_from_string` retains its panic behavior. Floating-point and rational constructors keep their explicit existing semantics; this series adds no infallible float conversion and no implicitly rounded rational conversion.

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
let a = Decimal::new_with_prec(100, 2);
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
let result = total.sub_owned(&b).into_rescaled(2, RoundingMode::HalfEven);
assert_eq!(result.string_with_trailing_zeros(), "12.35");
assert_eq!(a.to_string(), "12.35");
# Ok::<(), rs_decimal::DecimalError>(())
```

`add_owned(self, &rhs)` and `sub_owned(self, &rhs)` consume only the left operand and return an initialized result at max scale. `into_rescaled(self, prec, mode)` consumes the value with the same rules and panic conditions as rescale. Its unchanged-scale path still initializes default zero. These methods can reuse buffers, but growth, alignment and rounding may allocate.

Operator implementations are deliberately limited to `&Decimal + &Decimal`, `&Decimal - &Decimal`, and `-&Decimal`. Implementing the corresponding traits on owned Decimal could change legacy dot-method resolution when std::ops traits are imported. Explicit consuming methods avoid that source-compatibility hazard. There is no Mul/Div operator or default rounding policy: use mul_exact, mul(..., mode), or quo_with_prec(..., mode) as appropriate. AddAssign/SubAssign and a broad fallible arithmetic layer are outside this series.

## Prefer standard Rust spelling where semantics already match

| Compatibility entry | Rust usage / semantic note |
| --- | --- |
| new_from_int* at scale zero | `Decimal::from(integer)` |
| new_from_decimal | Move when the old binding is finished; otherwise Clone |
| new_from_string | `text.parse::<Decimal>()` |
| string | `d.to_string()` or `format!("{d}")` for canonical text; string_with_trailing_zeros retains stored scale |
| equal/not_equal/gt/gte/lt/lte | `==`, `!=`, `>`, `>=`, `<`, `<=` |
| cmp returning -1/0/1 | `Ord::cmp(&a, &b)` returns Ordering; the inherent cmp remains compatible |
| RoundingMode::from_raw | `RoundingMode::try_from(raw)` returns InvalidArgument for values outside 0..=6; from_raw still panics with its original string payload |
| safe_add/safe_sub/must_non_negative | These assert nonnegativity and may panic; they are not Result-returning checked operations |
| int64/uint64 | Success requires an exact, in-range integer; fractions return (0, false), and uint64 additionally rejects negatives |
| float32/float64 | The bool reports exactness of the floating-point value, not the integer-conversion success contract |
| r#mod / remainder | Prefer modulo for the signed arithmetic remainder; remainder() instead returns the integer part and unscaled fractional coefficient |

The scale of numerically equal values may differ. Default is uninitialized numerical zero, while normal arithmetic initializes results. NullDecimal.valid alone governs nullness, so its inner value may remain populated when invalid. Decoder methods may replace mutable receivers on success and retain the documented failure/null state rules. Neither an Option-based representation change nor a breaking rename/deprecation is part of this series.

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
| TryFrom<i32> for RoundingMode | tests/rust_api.rs complete 0..=6 mapping, invalid extremes and unchanged legacy panic payload |

Measured workloads and their limits are in [local copy measurements](performance-local-copies.md) and [owned pipeline measurements](performance-owned-api.md). Borrowing a reference and creating an independent value are different contracts; choose the interface according to the caller's ownership needs.
