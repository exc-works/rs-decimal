---
locale: en-GB
source_revision: dc01349bb451c857128d1cca5bf8b130f9b144b0e48f2d3acb755c072d9a7962
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::quo
---
# Decimal::quo

Divide decimals at an explicit or input-derived precision.

```rust
fn quo (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

other: &Decimal; Rust returns Result<Decimal, DecimalError>. Output scale is the larger operand scale. Two integer inputs therefore produce an integer: 1 / 3 with Down yields 0, while Up yields 1. Use quo_with_prec for fractional output. A zero divisor returns DivideByZero. mode controls rounding; Unnecessary returns RoundUnnecessary for an inexact result.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo" locale="en-GB" />

## Browser boundaries

Inputs are borrowed and remain usable after success or ordinary errors. Returned Decimals own independent data; arithmetic results are initialized. The browser checks the [resource policy](../resource-policy) before calling the core: alignment, intermediate products, division scaling and output precision. Conservative preflight may reject calculations with small final results. Resource errors remain distinct from numerical-library errors; a panic terminates the run.

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
