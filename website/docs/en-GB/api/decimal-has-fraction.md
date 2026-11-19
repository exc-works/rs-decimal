---
locale: en-GB
source_revision: e9defed137146374a74cf0b0a14760988cf4cd8ad97babd7fd11a73ccc7037b3
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::has_fraction
---
# Decimal::has_fraction

Test the numerical fractional component.

```rust
fn has_fraction (& self) -> bool
```

## Parameters, return and normal usage

No parameters; returns bool/boolean. This tests the numerical value, not whether scale is zero: 12.00 is an integer. Default zero is also an integer and has no fraction. These queries do not initialize or mutate the input.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-has-fraction" locale="en-GB" />

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
