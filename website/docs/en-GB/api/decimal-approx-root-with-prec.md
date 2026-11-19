---
locale: en-GB
source_revision: b69b3ecae1736be1a46c2a27882fc070a83e2b04381bee4c592838ebb67cda61
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::approx_root_with_prec
---
# Decimal::approx_root_with_prec

Compute a positive-degree root. degree: i64, transported as bigint; degree must be positive, otherwise InvalidRoot. Odd degrees accept negative input; even roots of negative values return NegativeRoot. Degree one is half-even rescaling. prec: isize, transported as an integer number; selects decimal places. Negative precision returns InvalidPrecision before domain checks. Rounding is half-even; exact midpoint roots such as sqrt(6.25) at precision zero round to 2. Despite the approx_root name, Rust certifies rounding using integer comparisons; exhausted core calculation/iteration budgets return NoConvergence. Input is borrowed and unchanged; results, including default zero, are initialized.

```rust
fn approx_root_with_prec (& self , degree : i64 , prec : isize) -> Result < Self , DecimalError >
```

## Normal usage

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-approx-root-with-prec" locale="en-GB" />

## Related methods

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
