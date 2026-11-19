---
locale: en-US
source_revision: e0028e1081b0309d3901334cb20b25aa450fe2c46a4c364db5668c9213b4477b
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

<MethodExample id="rounding-roundingmode-up" locale="en-US" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
