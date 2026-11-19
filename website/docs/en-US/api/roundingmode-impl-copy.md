---
locale: en-US
source_revision: ce0e6eba90e03ef0b3ea77c71d25c9216e58acb759f414a585091bbcd0fa1215
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

<MethodExample id="rounding-roundingmode-impl-copy" locale="en-US" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
