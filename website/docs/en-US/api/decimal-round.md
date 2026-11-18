---
locale: en-US
source_revision: dfbccd4ef90aecedf2d24565d11016e8068251bd103af1f82ea5d3b25f25cee0
api_id: Decimal::round
---
# Decimal::round

Round to nearest with ties to even to an integer.

```rust
fn round (& self) -> Self
```

## Parameters, return and normal usage

No parameters; returns a new Decimal at scale zero. Half-even is the default here; this is not JavaScript Math.round tie behavior. Inputs are borrowed and unchanged. Results are initialized, including a default-zero input. Increasing precision appends zeros without changing the numerical value.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round" locale="en-US" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
