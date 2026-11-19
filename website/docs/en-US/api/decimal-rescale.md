---
locale: en-US
source_revision: 203acbe634de9e56f24b2f6e3eccc13dcb25f721727e606d7d5a0efdca2a8e02
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

<MethodExample id="rounding-decimal-rescale" locale="en-US" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
