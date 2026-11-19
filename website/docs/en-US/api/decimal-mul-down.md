---
locale: en-US
source_revision: 8f612ca2b723227e5991f08e31b92d45cb4ea058472e86db2c02e8a1c4cc372e
api_id: Decimal::mul_down
---
# Decimal::mul_down

Multiply decimals with a defined output precision.

```rust
fn mul_down (& self , other : & Self) -> Self
```

## Parameters, return and normal usage

other: &Decimal. The result scale is the larger operand scale, not their sum. Use mul_exact to retain every product digit. Down truncates toward zero, including negative results.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-down" locale="en-US" />

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
