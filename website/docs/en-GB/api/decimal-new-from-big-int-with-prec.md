---
locale: en-GB
source_revision: bf618b8d345d150746683bfe2d04e5a166f48e4917e60db9f1d6a2719994d91a
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="constructor-new-from-big-int-with-prec" locale="en-GB" />

## Usage notes and boundaries

This changes the interpretation of the coefficient, not its digits. Negative precision is `InvalidPrecision`. The playground independently limits input digits and scale; `resource` errors do not indicate loss of numerical precision.

The browser also applies the [resource policy](../resource-policy). JavaScript inputs are validated before calling Rust.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
