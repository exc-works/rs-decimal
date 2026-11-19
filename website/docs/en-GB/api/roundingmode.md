---
locale: en-GB
source_revision: ac4a10a855b39a59f0636e888eaecef62de05250e4c407dbf6ae0398e83f87d9
translation_provenance: en-US-reviewed-reuse
api_id: RoundingMode
---
# RoundingMode

Select one of seven explicit rounding rules.

```rust
pub enum RoundingMode
```

## Parameters, return and normal usage

Use named enum variants in Rust and RoundingMode fields in the bridge. Rust discriminants 0–6 match the browser numbers. Down is toward zero; Up is away from zero; Ceiling is toward positive infinity. HalfUp/HalfDown/HalfEven resolve exact ties away from zero/toward zero/to even. Unnecessary requires exactness. There is no implicit global rounding setting.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode" locale="en-GB" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
