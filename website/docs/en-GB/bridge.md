---
locale: en-GB
source_revision: bd473c0431492bbeed88af5f453e401cc80b45775529394fc98a50e453c639ca
translation_provenance: en-US-reviewed-reuse
---
# Calling Rust methods in the browser

The playground runs JavaScript that calls the Rust library compiled to Wasm.
Method names, argument order, rounding and decimal state follow Rust. JavaScript
provides control flow; it does not calculate decimal results. See the [method catalogue](./api/) for individual behaviour and [Playground](./playground) to run examples.

```javascript
const price = Decimal.new_from_string('12.350');
const total = price.mul_exact(Decimal.new(3n));
total.string_with_trailing_zeros(); // '37.050'
```

Use exact strings for decimal input and `bigint` for i64 arguments (`3n`, not `3`).
`Decimal.new` rejects values outside i64, numeric strings and JS numbers. Precision
is a JS integer in the wasm32 signed range; fractional, nonfinite and out-of-range
inputs are rejected before an ABI cast can wrap/truncate them. A negative integer
precision reaches Rust and returns its original InvalidPrecision error.

## Errors and ownership

Rust returned errors throw an Error with `name: 'DecimalError'`, `category:
'library'`, the original `message`, and a `kinds` string array. Keep the full array:
a decoder error can match more than one kind. JS shape/range failures instead have
`name: 'BridgeError'` and `category: 'parameter'`; they are not Rust arithmetic
errors. Numeric invalid rounding discriminants reach Rust's validation.

Borrowed methods leave objects usable. `into_rescaled` consumes its receiver,
including when Rust returns an error. Using a consumed/disposed value raises a
`moved` BridgeError. Invalid JS argument shapes are rejected before the call and
do not consume the receiver. `clone()` preserves representation and initialization.
`free()` releases live handles and is harmless after consumption. Each runner
will discard its Wasm instance at completion, rather than relying on finalizers.
Objects from a different Playground session cannot be passed into a call.

## Nullable values and bytes

`NullDecimal.new(decimal)` consumes its argument and sets `valid=true`, even for
uninitialized zero. `NullDecimal.default()` is absent. Setting `valid` changes only
validity; it does not erase the inner value. `get_decimal_clone()` is an explicitly
named adapter returning an independent copy. `set_decimal(value)` moves a new inner
value without changing validity. There is no misleading live `.decimal` getter.

`coefficient()` similarly returns an immutable JS bigint snapshot. Rust borrow
lifetimes and compile-time trait checks are not reproduced by JavaScript.
`ByteBuffer.new(Uint8Array)` copies bytes into an owned buffer, and
`to_uint8_array()` returns a copy that survives Wasm memory growth. Append methods are documented in the [method catalogue](./api/).

`snapshot(value)` captures exact state immediately, including the
nullable's hidden value when invalid. It returns frozen data; later mutation does
not change prior output. The isolated runner bounds recursive output and execution time; see the [resource policy](./resource-policy).
