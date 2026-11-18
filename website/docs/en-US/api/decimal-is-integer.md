---
locale: en-US
source_revision: 6f40656db82d99e820bf25f176794a430859d18b708e5dfb0e4c5ad3ab2117e5
api_id: Decimal::is_integer
---
# Decimal::is_integer

Test whether the numerical value has no fractional component.

```rust
fn is_integer (& self) -> bool
```

## Parameters, return and normal usage

No parameters; returns bool/boolean. This tests the numerical value, not whether scale is zero: 12.00 is an integer. Default zero is also an integer and has no fraction. These queries do not initialize or mutate the input.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-is-integer" locale="en-US" />

## Browser boundaries

Inputs are borrowed and remain usable after success or ordinary errors. Returned Decimals own independent data; arithmetic results are initialized. The browser checks the [resource policy](../resource-policy) before calling the core: alignment, intermediate products, division scaling and output precision. Conservative preflight may reject calculations with small final results. Resource errors remain distinct from numerical-library errors; a panic terminates the run.

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
