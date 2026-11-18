---
locale: en-GB
source_revision: a19e9628caaaa2c66221e7031ee01f75dd38995c9d2873827f8d6308231f84ca
translation_provenance: en-US-reviewed-reuse
api_id: RoundingMode::HalfEven
---
# RoundingMode::HalfEven

Round nearest, with ties to an even retained digit.

```rust
HalfEven = 5
```

## Parameters, return and normal usage

Integer discriminant 5; use the named value instead of a magic number. The rule applies to both signs. Down and Up refer to zero, not negative/positive infinity. Half modes differ only at exact ties. There is no separate Floor enum; use floor_with_prec for that direction.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfeven" locale="en-GB" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
