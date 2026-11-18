---
locale: en-GB
source_revision: cf4512111f085d614e6ba264253c0be605dab554b0006351dc9a843e2de414f7
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::mul
---
# Decimal::mul

Multiply decimals with a defined output precision.

```rust
fn mul (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

other: &Decimal. The result scale is the larger operand scale, not their sum. Use mul_exact to retain every product digit. An explicit mode controls rounding; Unnecessary returns RoundUnnecessary when nonzero digits must be discarded.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul" locale="en-GB" />

## Browser boundaries

Inputs are borrowed and remain usable after success or ordinary errors. Returned Decimals own independent data; arithmetic results are initialized. The browser checks the [resource policy](../resource-policy) before calling the core: alignment, intermediate products, division scaling and output precision. Conservative preflight may reject calculations with small final results. Resource errors remain distinct from numerical-library errors; a panic terminates the run.

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
