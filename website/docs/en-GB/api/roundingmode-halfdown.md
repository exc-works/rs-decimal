---
locale: en-GB
source_revision: 80ca29e7b285c6bbe14fbaae400143c560d7d0375268641d09bff74efd458b6d
translation_provenance: en-US-reviewed-reuse
api_id: RoundingMode::HalfDown
---
# RoundingMode::HalfDown

Round nearest, with ties toward zero.

```rust
HalfDown = 4
```

## Parameters, return and normal usage

Integer discriminant 4; use the named value instead of a magic number. The rule applies to both signs. Down and Up refer to zero, not negative/positive infinity. Half modes differ only at exact ties. There is no separate Floor enum; use floor_with_prec for that direction.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfdown" locale="en-GB" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
