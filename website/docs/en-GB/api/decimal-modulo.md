---
locale: en-GB
source_revision: 4f9cc4e1f27490d354a3bbc2ca44814e0fd0d846ebf81964d518e23cc81067c0
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::modulo
---
# Decimal::modulo

Return the signed remainder of truncated division.

```rust
fn modulo (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

other: &Decimal; Rust returns Result<Decimal, DecimalError>. The remainder has the dividend sign and the larger operand scale; this is not a nonnegative Euclidean modulo. Zero divisor returns DivideByZero. Inputs are borrowed. Use quo_rem to obtain both quotient and remainder.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-modulo" locale="en-GB" />

## Browser boundaries

Follow the ownership rules above. Access after a move returns a moved error; free is idempotent. Type and alias validation precede a move, but Rust errors do not restore the consumed receiver. The [resource policy](../resource-policy) is separate from library errors: powers check intermediate values and alignment checks coefficient growth. Conservative checks can reject small final results.

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
