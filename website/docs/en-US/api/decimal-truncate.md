---
locale: en-US
source_revision: 2cd8f721f41f8104c290f5c405864c5ac1bec5cbcc73db293ddef64c58227cf7
api_id: Decimal::truncate
---
# Decimal::truncate

Round toward zero to an integer.

```rust
fn truncate (& self) -> Self
```

## Parameters, return and normal usage

No parameters; returns a new Decimal at scale zero. For negative values, truncation and floor differ: -1.5 truncates to -1 but floors to -2. Inputs are borrowed and unchanged. Results are initialized, including a default-zero input. Increasing precision appends zeros without changing the numerical value.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-truncate" locale="en-US" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
