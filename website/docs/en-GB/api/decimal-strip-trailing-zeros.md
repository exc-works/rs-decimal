---
locale: en-GB
source_revision: 584886d027ae1ffb7977be9193069c421cfe24c70d485c0551ed585a1d8e7407
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::strip_trailing_zeros
---
# Decimal::strip_trailing_zeros

Remove redundant fractional trailing zeros without changing the number.

```rust
fn strip_trailing_zeros (& self) -> Self
```

## Parameters, return and normal usage

No parameters; returns an independent Decimal. Factors of ten are removed only while scale is positive, so integer zeros such as 1200 are not discarded. Any numerical zero becomes initialized scale-zero zero.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-strip-trailing-zeros" locale="en-GB" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
