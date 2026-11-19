---
locale: en-US
source_revision: 636d6e9ec2d1b0cb7be987deb57dc622bb8084f92601482a3b6418a00ec64c8b
api_id: Decimal::round_with_prec
---
# Decimal::round_with_prec

Round to nearest with ties to even at a selected number of decimal places.

```rust
fn round_with_prec(&self, prec: isize) -> Result<Self, DecimalError>
```

## Parameter

`prec: isize` — Number of decimal places. The bridge accepts an integer JavaScript number.

## Return value

An initialized Decimal, including for a default-zero input. The input is borrowed. Increasing precision appends zeros.

`2.345 → 2.34` · `2.355 → 2.36` (HalfEven). Use [`rescale`](./decimal-rescale) to choose a different rounding mode.

## Errors

A negative precision returns `InvalidPrecision`. The bridge throws the corresponding error.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round-with-prec" locale="en-US" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
