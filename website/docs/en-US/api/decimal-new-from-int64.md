---
locale: en-US
source_revision: 2558a21d5d46dd2598e1fc940985c24d9012b531617d6e30f108a6afe7d17531
api_id: Decimal::new_from_int64
---
# Decimal::new_from_int64

Create a decimal from a signed 64-bit coefficient and decimal-place precision. This has the same numerical and state behavior as `new_with_prec`.

```rust
fn new_from_int64 (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## Parameters and return

`value: i64` → `bigint`; `prec: isize` → integer `number`. Returns `value × 10^-prec`.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-int64" locale="en-US" />

## Usage notes and boundaries

The value is a coefficient, not an already-scaled amount. Negative precision is `InvalidPrecision`; out-of-range i64 values are bridge parameter errors.

The browser also applies the [resource policy](../resource-policy). JavaScript inputs are validated before calling Rust.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
