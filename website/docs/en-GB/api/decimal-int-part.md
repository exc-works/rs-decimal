---
locale: en-GB
source_revision: 9743ba1e89051026e818a070b86183de419a65ca646afc51459afa17731968b9
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::int_part
---
# Decimal::int_part

Extract the integer part by truncating toward zero.

```rust
fn int_part (& self) -> BigInt
```

## Parameters, return and normal usage

No parameters; Rust returns BigInt and the bridge returns bigint, without floating-point conversion. Negative -12.75 yields -12, not floor(-12.75) = -13. Default zero yields zero. The example serializes the integer as a string to preserve arbitrary precision in JSON.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-int-part" locale="en-GB" />

## Browser boundaries

Inputs are borrowed and remain usable after success or ordinary errors. Returned Decimals own independent data; arithmetic results are initialized. The browser checks the [resource policy](../resource-policy) before calling the core: alignment, intermediate products, division scaling and output precision. Conservative preflight may reject calculations with small final results. Resource errors remain distinct from numerical-library errors; a panic terminates the run.

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
