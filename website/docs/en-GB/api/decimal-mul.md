---
locale: en-GB
source_revision: adde9dcc9ce010498ad91d2410621453cfaa363aae0ee4c3d4598482bb6eead0
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::mul
---
# Decimal::mul

Multiply decimals with a defined output precision.

```rust
fn mul (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

other: &Decimal. The result scale is the larger operand scale, not their sum. Use mul_exact to retain every product digit. An explicit mode controls rounding; Unnecessary returns RoundUnnecessary when nonzero digits must be discarded.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul" locale="en-GB" />

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
