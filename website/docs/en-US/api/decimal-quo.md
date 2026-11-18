---
locale: en-US
source_revision: 4842619da75eb5fbcf11cdce1b3ca781ba18031962688508c5f82ef39efdcf01
api_id: Decimal::quo
---
# Decimal::quo

Divide decimals at an explicit or input-derived precision.

```rust
fn quo (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

other: &Decimal; Rust returns Result<Decimal, DecimalError>. Output scale is the larger operand scale. Two integer inputs therefore produce an integer: 1 / 3 yields 0. Use quo_with_prec for fractional output. A zero divisor returns DivideByZero. mode controls rounding; Unnecessary returns RoundUnnecessary for an inexact result.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo" locale="en-US" />

## Browser boundaries

Inputs are borrowed and remain usable after success or ordinary errors. Returned Decimals own independent data; arithmetic results are initialized. The browser checks the [resource policy](../resource-policy) before calling the core: alignment, intermediate products, division scaling and output precision. Conservative preflight may reject calculations with small final results. Resource errors remain distinct from numerical-library errors; a panic terminates the run.

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
