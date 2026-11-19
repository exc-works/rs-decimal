---
locale: en-GB
source_revision: 845ec62ddce327ca5b797a7a47302c991bc3fbeb2f329134a32bdc0e5d88f53e
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::new_with_append_prec
---
# Decimal::new_with_append_prec

Keep a signed integer’s numerical value while storing additional decimal places. This is useful when a whole amount should be represented as 12.00.

```rust
fn new_with_append_prec (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## Parameters and return

`value: i64` → `bigint`; `prec: isize` → integer `number`. The stored coefficient becomes `value × 10^prec`; scale becomes `prec`.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-with-append-prec" locale="en-GB" />

## Usage notes

`string()` prints 12; `string_with_trailing_zeros()` prints 12.00. Unlike `new_with_prec`, the numerical value does not become 0.12. Negative precision is `InvalidPrecision`.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
