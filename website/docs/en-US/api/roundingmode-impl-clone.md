---
locale: en-US
source_revision: be26937ee4bcd1a30b10deb30b4ced946f0371e3015f1e537f8c85709084d801
api_id: RoundingMode::impl Clone
---
# RoundingMode::impl Clone

Use the Rust Clone implementation for rounding modes.

```rust
impl Clone for RoundingMode
```

## Parameters, return and normal usage

RoundingMode is a small Copy enum: copying preserves the selected rule. Equality compares variants, and Debug prints their names. This example validates native Rust trait syntax; browser modes are immutable numeric values.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-clone" locale="en-US" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
