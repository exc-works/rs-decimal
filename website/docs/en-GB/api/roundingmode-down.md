---
locale: en-GB
source_revision: be7e92c9ff29d88bdd7b2eadc18fb40911d696405ee369af6496615cdbec068e
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="rounding-roundingmode-down" locale="en-GB" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
