---
locale: en-US
source_revision: bb4dd0948b4d4be09c12115b5c13abd8421926284cdbcfed281913fc4636aeb5
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

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
