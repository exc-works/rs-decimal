---
locale: en-US
source_revision: 89b834d2c11952f4f6bb0f4503b5e891bf052db7e48a72abe6c367e79b9b8c68
api_id: Decimal::new_from_big_int_with_prec
---
# Decimal::new_from_big_int_with_prec

Copy an arbitrary-size coefficient and assign its decimal places. This keeps large integer input exact.

```rust
fn new_from_big_int_with_prec (value : & BigInt , prec : isize) -> Result < Self , DecimalError >
```

## Parameters and return

`value: &BigInt` → `bigint`; `prec: isize` → integer `number`. Returns `value × 10^-prec`, borrowing the Rust input.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-big-int-with-prec" locale="en-US" />

## Usage notes

This changes the interpretation of the coefficient, not its digits. Negative precision is `InvalidPrecision`.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
