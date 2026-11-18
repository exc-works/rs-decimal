---
locale: en-US
source_revision: 93dfb354f7906fd9c23e3b9ad99dace81c54c1bb0757216e3e8aa83d274f8bb9
api_id: Decimal::r#mod
---
# Decimal::r#mod

Return the signed remainder of truncated division.

```rust
fn r#mod (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

other: &Decimal; Rust returns Result<Decimal, DecimalError>. The remainder has the dividend sign and the larger operand scale; this is not a nonnegative Euclidean modulo. Zero divisor returns DivideByZero. Inputs are borrowed. Rust requires r#mod because mod is a keyword; JS uses .mod(other). Both names call the same core behavior.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-r-mod" locale="en-US" />

## Browser boundaries

Follow the ownership rules above. Access after a move returns a moved error; free is idempotent. Type and alias validation precede a move, but Rust errors do not restore the consumed receiver. The [resource policy](../resource-policy) is separate from library errors: powers check intermediate values and alignment checks coefficient growth. Conservative checks can reject small final results.

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
