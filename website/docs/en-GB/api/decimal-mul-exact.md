---
locale: en-GB
source_revision: 5fb085491b7b8fca3cac801efd640150df3603d315a453ad87ad49323fbb68e2
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

## Browser boundaries

Inputs are borrowed and remain usable after success or ordinary errors. Returned Decimals own independent data; arithmetic results are initialized. The browser checks the [resource policy](../resource-policy) before calling the core: alignment, intermediate products, division scaling and output precision. Conservative preflight may reject calculations with small final results. Resource errors remain distinct from numerical-library errors; a panic terminates the run.

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
