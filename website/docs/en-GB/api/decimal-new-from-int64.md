---
locale: en-GB
source_revision: 9ec084e5d70a80123e6b5391a460c2d09074b7e750d373283106219a5cde28ed
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="constructor-new-from-int64" locale="en-GB" />

## Usage notes

The value is a coefficient, not an already-scaled amount. Negative precision is `InvalidPrecision`; out-of-range i64 values are bridge parameter errors.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
