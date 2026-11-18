---
locale: en-GB
source_revision: 968f32ddfefb50259ee32498127a1ae440b4c11919fa84ceb9ba802a6f23f0ec
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::mul_down
---
# Decimal::mul_down

Multiply decimals with a defined output precision.

```rust
fn mul_down (& self , other : & Self) -> Self
```

## Parameters, return and normal usage

other: &Decimal. The result scale is the larger operand scale, not their sum. Use mul_exact to retain every product digit. Down truncates toward zero, including negative results.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-down" locale="en-GB" />

## Browser boundaries

Inputs are borrowed and remain usable after success or ordinary errors. Returned Decimals own independent data; arithmetic results are initialized. The browser checks the [resource policy](../resource-policy) before calling the core: alignment, intermediate products, division scaling and output precision. Conservative preflight may reject calculations with small final results. Resource errors remain distinct from numerical-library errors; a panic terminates the run.

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
