---
locale: en-US
source_revision: 340ee9c355dc0ac1e0c13e6d43aee222d595260389706ec4d9398cbe1a49147c
api_id: reexport::RoundingMode
---
# reexport::RoundingMode

Import rounding modes from the crate root.

```rust
pub use rounding::RoundingMode as RoundingMode
```

## Parameters, return and normal usage

Use rs_decimal::RoundingMode. This is the same enum, not a second implementation. The browser exposes the named RoundingMode object without Rust import syntax.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-reexport-roundingmode" locale="en-US" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
