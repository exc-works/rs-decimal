---
locale: en-GB
source_revision: bae692e3c5c12038fd1ef26d04bf1880cf624ae83c8c2ec0ad9f2f071f1b9f3a
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

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
