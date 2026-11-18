---
locale: en-US
source_revision: 240d5cd6140ed74d787cde34f7f3b1ea17105d751eeea756af72039c1ea346ce
api_id: Decimal::quo_with_prec
---
# Decimal::quo_with_prec

Divide decimals at an explicit or input-derived precision.

```rust
fn quo_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

other: &Decimal; Rust returns Result<Decimal, DecimalError>. prec selects nonnegative decimal places; negative precision returns InvalidPrecision before the divisor is checked. A zero divisor returns DivideByZero. mode controls rounding; Unnecessary returns RoundUnnecessary for an inexact result.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-with-prec" locale="en-US" />

## Browser boundaries

Inputs are borrowed and remain usable after success or ordinary errors. Returned Decimals own independent data; arithmetic results are initialized. The browser checks the [resource policy](../resource-policy) before calling the core: alignment, intermediate products, division scaling and output precision. Conservative preflight may reject calculations with small final results. Resource errors remain distinct from numerical-library errors; a panic terminates the run.

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
