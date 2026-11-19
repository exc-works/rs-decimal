---
locale: en-GB
source_revision: 823f73234f73cba4c9bfd0b5f1d418fa482625a8365337d1d27eb5d7306b4e6d
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

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
