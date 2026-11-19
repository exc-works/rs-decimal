---
locale: en-GB
source_revision: db43c880650b6eb59e9c805ec0cdb77a4879108bb98a5d2e10ec38f65ac55f46
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::sub
---
# Decimal::sub

Subtract decimals exactly.

```rust
fn sub (& self , other : & Self) -> Self
```

## Parameters, return and normal usage

other: &Decimal; returns a new Decimal at the larger operand scale. Alignment is exact and does not discard digits.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub" locale="en-GB" />

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
