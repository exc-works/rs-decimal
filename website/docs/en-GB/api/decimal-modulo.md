---
locale: en-GB
source_revision: 561bf3d1118c3ce8a6c2b94160186e43d5ce2153ae6df6a545c57b9a2538ed41
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

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
