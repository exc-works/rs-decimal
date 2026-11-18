---
locale: en-GB
source_revision: a66a15086c7943a9a541c1451a0fe544a096af2a9b938eb44bd20730eac8975d
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

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
