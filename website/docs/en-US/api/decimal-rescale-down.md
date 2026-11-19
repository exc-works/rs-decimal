---
locale: en-US
source_revision: e9dbb721c8987afdaedb4f74b013156db52924dc6bfa528e95669ba4396942ff
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

<MethodExample id="rounding-decimal-rescale-down" locale="en-US" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
