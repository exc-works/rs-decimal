---
locale: en-GB
source_revision: e228552c41e4946b118a075a0bdaa811f6ff511347af55baa0908ccb2c361609
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::rescale_down
---
# Decimal::rescale_down

Change decimal places by truncating toward zero.

```rust
fn rescale_down (& self , prec : isize) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

prec: isize/integer number; returns Decimal or InvalidPrecision for negative precision. Down means toward zero, not negative infinity. Increasing precision appends zeros. The input is unchanged and the result is initialized.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-rescale-down" locale="en-GB" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
