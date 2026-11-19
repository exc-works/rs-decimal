---
locale: en-US
source_revision: dce0eeed904ef266b7197b18fd411cf158683a2fe3f9074f19a11965111706e9
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

<MethodExample id="rounding-roundingmode-impl-partialeq" locale="en-US" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
