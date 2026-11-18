---
locale: en-GB
source_revision: 872ef2a43e7ac5d06f4ea120672471b0a862ee6f8a70e48db9f1f9367f2518f7
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::add_raw
---
# Decimal::add_raw

Change the unscaled coefficient by an i64 integer.

```rust
fn add_raw (& self , value : i64) -> Self
```

## Parameters, return and normal usage

value: i64, represented by bigint in the bridge. Scale is preserved: at scale 2, adding/subtracting 1 changes the number by 0.01, not by 1. Rust returns a new Decimal. Numbers and out-of-i64 bigint values are rejected by the bridge.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add-raw" locale="en-GB" />

## Browser boundaries

Inputs are borrowed and remain usable after success or ordinary errors. Returned Decimals own independent data; arithmetic results are initialized. The browser checks the [resource policy](../resource-policy) before calling the core: alignment, intermediate products, division scaling and output precision. Conservative preflight may reject calculations with small final results. Resource errors remain distinct from numerical-library errors; a panic terminates the run.

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
