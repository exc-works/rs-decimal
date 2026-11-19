---
locale: en-GB
source_revision: 6ba2cb4a20d5541fd7637c273e7892b620d22f9a54a85ae9d00d1d5387296c75
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::floor_with_prec
---
# Decimal::floor_with_prec

Round toward negative infinity at a selected number of decimal places.

```rust
fn floor_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

prec: isize/integer number. Negative precision is InvalidPrecision. For negative values, truncation and floor differ: -1.5 truncates to -1 but floors to -2. Inputs are borrowed and unchanged. Results are initialized, including a default-zero input. Increasing precision appends zeros without changing the numerical value.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-floor-with-prec" locale="en-GB" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
