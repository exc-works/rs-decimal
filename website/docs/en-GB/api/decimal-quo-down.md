---
locale: en-GB
source_revision: 96feaba7858db9f7e7efab2f2187c38abada064aba6bc3c66092f84f0b94b74f
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::quo_down
---
# Decimal::quo_down

Divide decimals at an explicit or input-derived precision.

```rust
fn quo_down (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

other: &Decimal; Rust returns Result<Decimal, DecimalError>. Output scale is the larger operand scale. Two integer inputs therefore produce an integer: 1 / 3 yields 0. Use quo_with_prec for fractional output. A zero divisor returns DivideByZero. Down truncates toward zero, not toward negative infinity.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-down" locale="en-GB" />

## Browser boundaries

Inputs are borrowed and remain usable after success or ordinary errors. Returned Decimals own independent data; arithmetic results are initialized. The browser checks the [resource policy](../resource-policy) before calling the core: alignment, intermediate products, division scaling and output precision. Conservative preflight may reject calculations with small final results. Resource errors remain distinct from numerical-library errors; a panic terminates the run.

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
