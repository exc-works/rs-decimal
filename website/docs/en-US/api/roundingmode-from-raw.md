---
locale: en-US
source_revision: 16147351e8fe026905ac983886e4a94a6ce341eb3c301ee1901cd0719ea82ee1
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

<MethodExample id="rounding-roundingmode-from-raw" locale="en-US" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
