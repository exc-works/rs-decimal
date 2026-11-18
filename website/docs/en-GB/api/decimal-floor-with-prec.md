---
locale: en-GB
source_revision: 2a8346ce45dbd45b20992f8d2634279ac1b05e18f68e660feb98bc0c9b86803d
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::floor_with_prec
---
# Decimal::floor_with_prec

Round toward negative infinity at a selected number of decimal places.

```rust
fn floor_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

prec: isize/integer number; returns Result<Decimal, DecimalError> in Rust and Decimal or an exception in the bridge. Negative precision is InvalidPrecision. For negative values, truncation and floor differ: -1.5 truncates to -1 but floors to -2. Inputs are borrowed and unchanged. Results are initialized, including a default-zero input. Increasing precision appends zeros without changing the numerical value.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-floor-with-prec" locale="en-GB" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
