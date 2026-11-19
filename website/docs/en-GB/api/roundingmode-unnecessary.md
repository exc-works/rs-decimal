---
locale: en-GB
source_revision: 50372ca1028f4ab77e1e376da48c589b0fdad5a066b7c436d3b5780901b17b77
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="rounding-roundingmode-unnecessary" locale="en-GB" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
