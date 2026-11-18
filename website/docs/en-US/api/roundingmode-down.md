---
locale: en-US
source_revision: 1357a444fcbe92d04b062a936583d9484c1caa7bb3d9db8faf200a33573fd049
api_id: RoundingMode::Down
---
# RoundingMode::Down

Round toward zero.

```rust
Down = 0
```

## Parameters, return and normal usage

Integer discriminant 0; use the named value instead of a magic number. The rule applies to both signs. Down and Up refer to zero, not negative/positive infinity. Half modes differ only at exact ties. There is no separate Floor enum; use floor_with_prec for that direction.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-down" locale="en-US" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
