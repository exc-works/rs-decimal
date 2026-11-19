---
locale: en-GB
source_revision: 6f37682196df5b201aacde1b52aa54d192c469e0d50cfead6f78c24b12704f29
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::truncate_with_prec
---
# Decimal::truncate_with_prec

Round toward zero at a selected number of decimal places.

```rust
fn truncate_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

prec: isize/integer number; returns Result<Decimal, DecimalError> in Rust and Decimal or an exception in the bridge. Negative precision is InvalidPrecision. For negative values, truncation and floor differ: -1.5 truncates to -1 but floors to -2. Inputs are borrowed and unchanged. Results are initialized, including a default-zero input. Increasing precision appends zeros without changing the numerical value.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-truncate-with-prec" locale="en-GB" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
