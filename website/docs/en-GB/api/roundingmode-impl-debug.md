---
locale: en-GB
source_revision: 22b226c9626e0bfe9b6ad82b4e6efbc955c78f9e4b48acf530ad413ce7ec7c69
translation_provenance: en-US-reviewed-reuse
api_id: RoundingMode::impl Debug
---
# RoundingMode::impl Debug

Use the Rust Debug implementation for rounding modes.

```rust
impl Debug for RoundingMode
```

## Parameters, return and normal usage

RoundingMode is a small Copy enum: copying preserves the selected rule. Equality compares variants, and Debug prints their names. This example validates native Rust trait syntax; browser modes are immutable numeric values.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-debug" locale="en-GB" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
