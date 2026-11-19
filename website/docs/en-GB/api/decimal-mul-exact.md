---
locale: en-GB
source_revision: 74fe800675809309f5a6802d09df41ce125ec348c7dc823b0a9a763856b0bb52
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::mul_exact
---
# Decimal::mul_exact

Multiply decimals exactly.

```rust
fn mul_exact (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

other: &Decimal. The result scale is the sum of operand scales; every product digit is retained. Core scale overflow returns Overflow. There is no rounding-mode parameter.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-exact" locale="en-GB" />

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
