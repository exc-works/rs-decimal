---
locale: en-GB
source_revision: 24cee5102dc473cb8fed8902e9829dd352863649ca89859b6c19bb3f001aa710
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

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
