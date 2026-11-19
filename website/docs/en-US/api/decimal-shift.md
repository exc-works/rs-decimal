---
locale: en-US
source_revision: 50b84c140827ca9ee14623ef754490d47ed0de164e4d5e55011356b86e00545d
api_id: Decimal::shift
---
# Decimal::shift

Multiply a decimal by an exact power of ten.

```rust
fn shift (& self , places : isize) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

places: isize/integer number. Positive places move the decimal point right; negative places move it left. This changes the numerical value, unlike rescale. The core returns Overflow when a nonzero shift produces scale above i32::MAX. Even shift(0) initializes default zero.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-shift" locale="en-US" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
