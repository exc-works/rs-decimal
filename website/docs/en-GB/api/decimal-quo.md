---
locale: en-GB
source_revision: 0d6262b2eb7c6955f187e4d63b88d16a39d035f514ca55c738d74b10a8d69203
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

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
