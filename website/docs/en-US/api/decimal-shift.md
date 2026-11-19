---
locale: en-US
source_revision: 6d142adfffddbbc67008e17859ebe097a896ff7983bb306596fd170c8b867b28
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

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
