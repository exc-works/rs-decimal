---
locale: en-GB
source_revision: aec9d4c9d2a5eeca4a577ca8099051ca021bc718a47c3458f4e1e7ab0bca048b
translation_provenance: en-US-reviewed-reuse
api_id: RoundingMode::Up
---
# RoundingMode::Up

Round away from zero.

```rust
Up = 1
```

## Parameters, return and normal usage

Integer discriminant 1; use the named value instead of a magic number. The rule applies to both signs. Down and Up refer to zero, not negative/positive infinity. Half modes differ only at exact ties. There is no separate Floor enum; use floor_with_prec for that direction.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-up" locale="en-GB" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
