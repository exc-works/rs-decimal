---
locale: en-US
source_revision: b1d5b92283e0de37c46d70318c8e896d60bc04df8806e365913a4c12caa7a8d9
api_id: Decimal::ceil
---
# Decimal::ceil

Round toward positive infinity to an integer.

```rust
fn ceil (& self) -> Self
```

## Parameters, return and normal usage

No parameters; returns a new Decimal at scale zero. Inputs are borrowed and unchanged. Results are initialized, including a default-zero input. Increasing precision appends zeros without changing the numerical value.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil" locale="en-US" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
