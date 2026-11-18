---
locale: en-US
source_revision: 9cb35d36232b48dfa841cf9ea23e2fcf6d9b36b6ef45c2f68538158901813e1b
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

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
