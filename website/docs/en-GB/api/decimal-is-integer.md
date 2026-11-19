---
locale: en-GB
source_revision: e73578c50b0181c9f243a7f97c0c8dbdcdf0b57587c7ac98a36dc6043a2ce666
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::is_integer
---
# Decimal::is_integer

Test whether the numerical value has no fractional component.

```rust
fn is_integer (& self) -> bool
```

## Parameters, return and normal usage

No parameters; returns bool/boolean. This tests the numerical value, not whether scale is zero: 12.00 is an integer. Default zero is also an integer and has no fraction. These queries do not initialize or mutate the input.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-is-integer" locale="en-GB" />

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
