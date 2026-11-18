---
locale: en-GB
source_revision: 4689fa28c94f2db26b471c2a8694afab36dc5eb945d0739ba6a3d98fb3e08c56
translation_provenance: en-US-reviewed-reuse
api_id: RoundingMode::impl Copy
---
# RoundingMode::impl Copy

Use the Rust Copy implementation for rounding modes.

```rust
impl Copy for RoundingMode
```

## Parameters, return and normal usage

RoundingMode is a small Copy enum: copying preserves the selected rule. Equality compares variants, and Debug prints their names. This example validates native Rust trait syntax; browser modes are immutable numeric values.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-copy" locale="en-GB" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
