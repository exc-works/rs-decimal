---
locale: en-GB
source_revision: 10c5c67f0eca13950935316848dbaf7b4f6fbe37a761aaabb0a34ec3fe68d1c2
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::quo_rem
---
# Decimal::quo_rem

Return an integer quotient and signed Decimal remainder.

```rust
fn quo_rem (& self , other : & Self) -> Result < (Self , Self) , DecimalError >
```

## Parameters, return and normal usage

other: &Decimal; Rust returns Result<(Decimal, Decimal), DecimalError>, mapped to [quotient, remainder]. Quotient truncates toward zero at scale zero. Remainder has the dividend sign and the larger operand scale, satisfying self = quotient * other + remainder. Each result is an independent initialized wrapper; inputs remain usable. A zero divisor returns DivideByZero before alignment.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-quo-rem" locale="en-GB" />

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
