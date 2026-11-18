---
locale: en-GB
source_revision: 24579cc5f98ad85ad1558995873e9949e2e4e6ae6e01cc4702b2ee7ca18a14ea
translation_provenance: en-US-reviewed-reuse
api_id: RoundingMode
---
# RoundingMode

Select one of seven explicit rounding rules.

```rust
pub enum RoundingMode
```

## Parameters, return and normal usage

Use named enum variants in Rust and RoundingMode fields in the bridge. Rust discriminants 0–6 match the browser numbers. Down is toward zero; Up is away from zero; Ceiling is toward positive infinity. HalfUp/HalfDown/HalfEven resolve exact ties away from zero/toward zero/to even. Unnecessary requires exactness. There is no implicit global rounding setting.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode" locale="en-GB" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
