---
locale: en-GB
source_revision: d588c3aa025efc49b40f1c2c9f6f76c0ff1df249830b32d566fb6a6a5794cdc5
translation_provenance: en-US-reviewed-reuse
api_id: RoundingMode::impl Eq
---
# RoundingMode::impl Eq

Use the Rust Eq implementation for rounding modes.

```rust
impl Eq for RoundingMode
```

## Parameters, return and normal usage

RoundingMode is a small Copy enum: copying preserves the selected rule. Equality compares variants, and Debug prints their names. This example validates native Rust trait syntax; browser modes are immutable numeric values.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-eq" locale="en-GB" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
