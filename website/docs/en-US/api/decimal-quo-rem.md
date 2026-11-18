---
locale: en-US
source_revision: ab458b72e36df078d37dd6de4be21a55e10d1e6c955c47b4abcbe87cbbdd8e23
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

<MethodExample id="owned-decimal-quo-rem" locale="en-US" />

## Browser boundaries

Follow the ownership rules above. Access after a move returns a moved error; free is idempotent. Type and alias validation precede a move, but Rust errors do not restore the consumed receiver. The [resource policy](../resource-policy) is separate from library errors: powers check intermediate values and alignment checks coefficient growth. Conservative checks can reject small final results.

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
