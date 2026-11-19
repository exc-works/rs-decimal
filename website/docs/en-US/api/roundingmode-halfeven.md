---
locale: en-US
source_revision: 84ba3002f85664068a9d9ececc408a7d6f0214a486be55b19a41d101ddf845eb
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

<MethodExample id="rounding-roundingmode-halfeven" locale="en-US" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
