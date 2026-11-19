---
locale: en-US
source_revision: b1efdab89860dbce37ea848eed30ac657af5e945bd5643b2378d7d893a2ce9a2
api_id: RoundingMode::from_raw
---
# RoundingMode::from_raw

Validate an integer rounding-mode code.

```rust
fn from_raw (value : i32) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

value: i32/integer number. Returns the mode for 0–6; other i32 values return InvalidArgument. Fractional, nonfinite or out-of-i32 browser values fail parameter validation before Rust. The JavaScript API calls the Rust validator and returns the numeric mode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-from-raw" locale="en-US" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
