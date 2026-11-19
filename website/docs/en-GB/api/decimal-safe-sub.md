---
locale: en-GB
source_revision: b834c8dbe54adc1326f4715bae0b5a7e4a551bc4817e951238d3a76cb1960aa5
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::safe_sub
---
# Decimal::safe_sub

Subtract decimals exactly and require a nonnegative result.

```rust
fn safe_sub (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

other: &Decimal; returns a new Decimal at the larger operand scale. Alignment is exact and does not discard digits. Rust returns Result: only the result must be nonnegative, not each operand. A negative result returns InvalidArgument; zero is allowed.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-sub" locale="en-GB" />

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
