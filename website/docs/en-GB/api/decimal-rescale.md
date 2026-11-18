---
locale: en-GB
source_revision: 0deb49519f3f009c2dba973109de20fc0fcdfe90b9617749c506123c4c387997
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::rescale
---
# Decimal::rescale

Change decimal places with an explicit rounding mode.

```rust
fn rescale (& self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

prec is decimal places, not significant digits; mode is one of the seven RoundingMode values. Increasing precision appends zeros. Decreasing precision rounds only discarded nonzero digits. Negative precision returns InvalidPrecision; Unnecessary returns RoundUnnecessary when rounding is required. Results initialize default zero; borrowed inputs remain unchanged.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-rescale" locale="en-GB" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
