---
locale: en-US
source_revision: 1d47fa6ae1a12d0a01d3c07d39948787bbb4209bfed6d0edc8d1a4933e938a92
api_id: RoundingMode::Unnecessary
---
# RoundingMode::Unnecessary

Round exact results only.

```rust
Unnecessary = 6
```

## Parameters, return and normal usage

Integer discriminant 6; use the named value instead of a magic number. The rule applies to both signs. Discarding zero digits is exact; discarding any nonzero digit returns RoundUnnecessary.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-unnecessary" locale="en-US" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
