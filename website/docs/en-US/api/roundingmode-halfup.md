---
locale: en-US
source_revision: e0f36bf4a4b64a112b7758ba3b58ccfab48769ae252409c4b72638890e0e2a0e
api_id: RoundingMode::HalfUp
---
# RoundingMode::HalfUp

Round nearest, with ties away from zero.

```rust
HalfUp = 3
```

## Parameters, return and normal usage

Integer discriminant 3; use the named value instead of a magic number. The rule applies to both signs. Down and Up refer to zero, not negative/positive infinity. Half modes differ only at exact ties. There is no separate Floor enum; use floor_with_prec for that direction.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfup" locale="en-US" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
