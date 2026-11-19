---
locale: en-US
source_revision: eb9bcc338f948ddd5ec14bfd00bb5bb38695bb5e5eba9b781fe855d89569b0a0
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

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
