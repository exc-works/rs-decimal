---
locale: en-US
source_revision: 5a350c30c073c772c4921158a8a0d829d4480ae375ec35f7532a213f3f45e557
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

<MethodExample id="constructor-new-with-append-prec" locale="en-US" />

## Usage notes

`string()` prints 12; `string_with_trailing_zeros()` prints 12.00. Unlike `new_with_prec`, the numerical value does not become 0.12. Negative precision is `InvalidPrecision`; large requests are rejected before allocating powers of ten.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
