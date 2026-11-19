---
locale: en-GB
source_revision: 53cafbe90f3ab017866bcf2afb140d94d6b1a293f7aa6ca2a32a775110c8f82c
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::ceil
---
# Decimal::ceil

Round toward positive infinity to an integer.

```rust
fn ceil (& self) -> Self
```

## Parameters, return and normal usage

No parameters; returns a new Decimal at scale zero. Inputs are borrowed and unchanged. Results are initialized, including a default-zero input. Increasing precision appends zeros without changing the numerical value.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil" locale="en-GB" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
