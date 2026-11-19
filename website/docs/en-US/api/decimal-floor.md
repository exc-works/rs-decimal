---
locale: en-US
source_revision: db004727284a336c6f0e549dbc589384cfceb8780f95c6036a8917d02c730891
api_id: Decimal::floor
---
# Decimal::floor

Round toward negative infinity to an integer.

```rust
fn floor (& self) -> Self
```

## Parameters, return and normal usage

No parameters; returns a new Decimal at scale zero. For negative values, truncation and floor differ: -1.5 truncates to -1 but floors to -2. Inputs are borrowed and unchanged. Results are initialized, including a default-zero input. Increasing precision appends zeros without changing the numerical value.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-floor" locale="en-US" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
