---
locale: en-US
source_revision: 197565e31e34527cbc33f567c65ff6fb5c5304500454cb2da830919e852c5158
api_id: Decimal::safe_add
---
# Decimal::safe_add

Add decimals exactly and require a nonnegative result.

```rust
fn safe_add (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

other: &Decimal; returns a new Decimal at the larger operand scale. Alignment is exact and does not discard digits. Rust returns Result: only the result must be nonnegative, not each operand. A negative result returns InvalidArgument; zero is allowed.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-add" locale="en-US" />

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
