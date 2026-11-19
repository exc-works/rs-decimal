---
locale: en-US
source_revision: b2363400afff1e4c740938a16816a148faaff0faef10b304c45a776ef2dcc1ee
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

<MethodExample id="arithmetic-decimal-mul-with-prec" locale="en-US" />

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
