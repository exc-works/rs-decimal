---
locale: en-US
source_revision: d6f697a3d962962b5c45eb345607fbc6629bac0f2d7f0f5c403da5649a7d1d4a
api_id: Decimal::round_with_prec
---
# Decimal::round_with_prec

Round to nearest with ties to even at a selected number of decimal places.

```rust
fn round_with_prec(&self, prec: isize) -> Result<Self, DecimalError>
```

## Parameter

`prec: isize` — Number of decimal places (integer).

## Return value

An initialized Decimal, including for a default-zero input. The input is borrowed. Increasing precision appends zeros.

`2.345 → 2.34` · `2.355 → 2.36` (HalfEven). Use [`rescale`](./decimal-rescale) to choose a different rounding mode.

## Errors

A negative precision returns `InvalidPrecision`.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round-with-prec" locale="en-US" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
