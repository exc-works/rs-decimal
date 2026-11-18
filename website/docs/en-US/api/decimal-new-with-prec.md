---
locale: en-US
source_revision: 78d4bcb73439eeefbcaa373b59f6572230d9fd809bde6e3821566e584e60e547
api_id: Decimal::new_with_prec
---
# Decimal::new_with_prec

Interpret a signed integer as a coefficient at a decimal-place precision. For example, 12 at precision 2 means 0.12.

```rust
fn new_with_prec (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## Parameters and return

`value: i64` → `bigint`; `prec: isize` → integer `number`. Returns `value × 10^-prec`.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-with-prec" locale="en-US" />

## Usage notes and boundaries

Precision counts decimal places, not significant digits. This does not append zeros to an unchanged integer; use `new_with_append_prec` for 12.00. Negative precision is `InvalidPrecision`. A zero coefficient retains the requested scale.

The browser also applies the [resource policy](../resource-policy). JavaScript inputs are validated before calling Rust.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
