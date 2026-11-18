---
locale: en-GB
source_revision: 6c8f91b0409cc71714a4b9926acf34a3cf0aa635f8904f2e607ed3243fb8b30a
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::power
---
# Decimal::power

Raise to an integer power at the original scale.

```rust
fn power (& self , power : i64) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

power: i64, transported as bigint. Rounds half-even to the input scale; negative powers take the reciprocal. 2.00 to -3 yields 0.12, while integer-scale 2 to -3 yields 0. Zero to zero is initialized one; zero to a negative power returns DivideByZero. Huge exponents that exceed the core intermediate bound return Overflow before allocation. The browser has smaller intermediate scale/coefficient budgets; 0 and +/-1 retain the core fast paths. The input is borrowed.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-power" locale="en-GB" />

## Browser boundaries

Follow the ownership rules above. Access after a move returns a moved error; free is idempotent. Type and alias validation precede a move, but Rust errors do not restore the consumed receiver. The [resource policy](../resource-policy) is separate from library errors: powers check intermediate values and alignment checks coefficient growth. Conservative checks can reject small final results.

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
