---
locale: en-US
source_revision: 8fb2de8eaa72e1390b09fd949ca444ffb681b4ab0d4c0a3535431123d0666727
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

<MethodExample id="rounding-decimal-floor-with-prec" locale="en-US" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
