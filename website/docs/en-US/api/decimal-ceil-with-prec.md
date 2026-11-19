---
locale: en-US
source_revision: 11b8aa485f155a658e5960045fe97e0878f07fd6eb371a471f1440f29be38903
api_id: Decimal::ceil_with_prec
---
# Decimal::ceil_with_prec

Round toward positive infinity at a selected number of decimal places.

```rust
fn ceil_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

prec: isize/integer number. Negative precision is InvalidPrecision. Inputs are borrowed and unchanged. Results are initialized, including a default-zero input. Increasing precision appends zeros without changing the numerical value.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil-with-prec" locale="en-US" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
