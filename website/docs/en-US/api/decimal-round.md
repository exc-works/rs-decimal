---
locale: en-US
source_revision: 6e97ceecb692f3b30f694df6cf3812a7bc93e427ba0e4c690447e33c151380a9
api_id: Decimal::round
---
# Decimal::round

Round to nearest with ties to even to an integer.

```rust
fn round (& self) -> Self
```

## Parameters, return and normal usage

No parameters; returns a new Decimal at scale zero. Half-even is the default here; this is not JavaScript Math.round tie behavior. Inputs are borrowed and unchanged. Results are initialized, including a default-zero input. Increasing precision appends zeros without changing the numerical value.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round" locale="en-US" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
