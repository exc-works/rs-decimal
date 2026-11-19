---
locale: en-US
source_revision: 53a165ed47764f5f661387617a1b68ba2abdefa181cac1cde270d551da87797c
api_id: Decimal::sqrt_with_prec
---
# Decimal::sqrt_with_prec

Compute a square root. No degree parameter. Negative input returns NegativeRoot. prec: isize, transported as an integer number; selects decimal places. Negative precision returns InvalidPrecision before domain checks. Rounding is half-even; exact midpoint roots such as sqrt(6.25) at precision zero round to 2. Despite the approx_root name, Rust certifies rounding using integer comparisons; exhausted core calculation/iteration budgets return NoConvergence. Input is borrowed and unchanged; results, including default zero, are initialized.

```rust
fn sqrt_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Normal usage

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-sqrt-with-prec" locale="en-US" />

## Related methods

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
