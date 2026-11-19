---
locale: en-US
source_revision: d9bfbd2d29924d270b97f6a4251656ca421cb89e51e3bdcdc1ecdf2cdefab10f
api_id: Decimal::truncate_with_prec
---
# Decimal::truncate_with_prec

Round toward zero at a selected number of decimal places.

```rust
fn truncate_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

prec: isize/integer number. Negative precision is InvalidPrecision. For negative values, truncation and floor differ: -1.5 truncates to -1 but floors to -2. Inputs are borrowed and unchanged. Results are initialized, including a default-zero input. Increasing precision appends zeros without changing the numerical value.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-truncate-with-prec" locale="en-US" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
