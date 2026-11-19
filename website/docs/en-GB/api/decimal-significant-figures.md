---
locale: en-GB
source_revision: 6155fe36a4ec8e1b03a89c20c5c7247a068af20681f3b463de98b38c447a2097
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::significant_figures
---
# Decimal::significant_figures

Round to at most a selected number of significant decimal digits.

```rust
fn significant_figures (& self , figures : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

figures: positive isize/integer number; mode selects rounding. Unlike decimal-place precision, significant figures count coefficient digits. This never adds precision and may round integer digits (1234.5 → 1230 with three figures). Nonpositive figures return InvalidArgument; required Unnecessary rounding returns RoundUnnecessary. A possible coefficient carry is checked against the browser budget first.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-significant-figures" locale="en-GB" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
