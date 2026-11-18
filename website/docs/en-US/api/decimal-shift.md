---
locale: en-US
source_revision: 69f5b453b56bcf2350f3a3cf2e3d29b1035a464f2105cefeff33c4c49c926524
api_id: Decimal::shift
---
# Decimal::shift

Multiply a decimal by an exact power of ten.

```rust
fn shift (& self , places : isize) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

places: isize/integer number. Positive places move the decimal point right; negative places move it left. This changes the numerical value, unlike rescale. The core returns Overflow when a nonzero shift produces scale above i32::MAX. Browser growth/scale checks can instead return a separate resource error before allocation. Even shift(0) initializes default zero.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-shift" locale="en-US" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
