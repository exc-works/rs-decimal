---
locale: en-GB
source_revision: 32a89fd803ef33f414f98138648ca1bbfe7cf03c12443bee715ec01ffb382e0d
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::floor
---
# Decimal::floor

Round toward negative infinity to an integer.

```rust
fn floor (& self) -> Self
```

## Parameters, return and normal usage

No parameters; returns a new Decimal at scale zero. For negative values, truncation and floor differ: -1.5 truncates to -1 but floors to -2. Inputs are borrowed and unchanged. Results are initialized, including a default-zero input. Increasing precision appends zeros without changing the numerical value.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-floor" locale="en-GB" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
