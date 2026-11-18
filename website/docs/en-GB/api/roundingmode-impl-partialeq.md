---
locale: en-GB
source_revision: d358fea1722186e0df2e76e232b3793bff18d2e3a10cb6ef0ae0a724a40da40b
translation_provenance: en-US-reviewed-reuse
api_id: RoundingMode::impl PartialEq
---
# RoundingMode::impl PartialEq

Use the Rust PartialEq implementation for rounding modes.

```rust
impl PartialEq for RoundingMode
```

## Parameters, return and normal usage

RoundingMode is a small Copy enum: copying preserves the selected rule. Equality compares variants, and Debug prints their names. This example validates native Rust trait syntax; browser modes are immutable numeric values.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-partialeq" locale="en-GB" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
