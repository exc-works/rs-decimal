---
locale: en-GB
source_revision: 4f94d3a29039b5504e83148308232156a8a140f150fc1e38d0956e8fb963c92b
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::has_fraction
---
# Decimal::has_fraction

Test the numerical fractional component.

```rust
fn has_fraction (& self) -> bool
```

## Parameters, return and normal usage

No parameters; returns bool/boolean. This tests the numerical value, not whether scale is zero: 12.00 is an integer. Default zero is also an integer and has no fraction. These queries do not initialize or mutate the input.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-has-fraction" locale="en-GB" />

## Browser boundaries

Inputs are borrowed and remain usable after success or ordinary errors. Returned Decimals own independent data; arithmetic results are initialized. The browser checks the [resource policy](../resource-policy) before calling the core: alignment, intermediate products, division scaling and output precision. Conservative preflight may reject calculations with small final results. Resource errors remain distinct from numerical-library errors; a panic terminates the run.

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
