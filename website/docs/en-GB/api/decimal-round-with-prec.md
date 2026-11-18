---
locale: en-GB
source_revision: 08a6704725cb461888907af8e46adcebe3aa92b6c462154ff5ba96f3a955340f
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::round_with_prec
---
# Decimal::round_with_prec

Round to nearest with ties to even at a selected number of decimal places.

```rust
fn round_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

prec: isize/integer number; returns Result<Decimal, DecimalError> in Rust and Decimal or an exception in the bridge. Negative precision is InvalidPrecision. Half-even is the default here; this is not JavaScript Math.round tie behavior. Inputs are borrowed and unchanged. Results are initialized, including a default-zero input. Increasing precision appends zeros without changing the numerical value.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round-with-prec" locale="en-GB" />

## Browser boundaries

Integer parameters must fit the wasm32 range. The browser additionally applies the [resource policy](../resource-policy), including target scale, coefficient growth and call count. Resource errors remain distinct from numerical-library errors. Borrowed methods do not consume inputs; Rust-only trait syntax is explicitly labeled as native-tested.

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
