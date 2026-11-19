---
locale: en-GB
source_revision: e9709f45197c4a553d9eef583ec3ace47a8ab16090d52890478f7fffb689b911
translation_provenance: en-US-reviewed-reuse
api_id: RoundingMode::from_raw
---
# RoundingMode::from_raw

Validate an integer rounding-mode code.

```rust
fn from_raw (value : i32) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

value: i32/integer number. Returns the mode for 0–6; other i32 values return InvalidArgument. Fractional, nonfinite or out-of-i32 browser values fail parameter validation before Rust. The bridge calls the Rust validator and returns the numeric mode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-from-raw" locale="en-GB" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
