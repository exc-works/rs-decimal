---
locale: en-US
source_revision: 01c476851989c18cecc0970b361010eee7cb4cfd902d7e4240be18102abc0899
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

<MethodExample id="owned-decimal-power" locale="en-US" />

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
