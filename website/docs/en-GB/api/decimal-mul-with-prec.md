---
locale: en-GB
source_revision: ce941de7d4ef355a538d2c2c33a609ab4eeb3b4645eb801e81a24755fcf0ca5a
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::mul_with_prec
---
# Decimal::mul_with_prec

Multiply decimals with a defined output precision.

```rust
fn mul_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

other: &Decimal. The result scale is prec decimal places, which must be nonnegative; negative prec returns InvalidPrecision. An explicit mode controls rounding; Unnecessary returns RoundUnnecessary when nonzero digits must be discarded.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-with-prec" locale="en-GB" />

## Browser boundaries

Inputs are borrowed and remain usable after success or ordinary errors. Returned Decimals own independent data; arithmetic results are initialized. The browser checks the [resource policy](../resource-policy) before calling the core: alignment, intermediate products, division scaling and output precision. Conservative preflight may reject calculations with small final results. Resource errors remain distinct from numerical-library errors; a panic terminates the run.

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
