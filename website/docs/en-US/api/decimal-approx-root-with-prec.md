---
locale: en-US
source_revision: 3553acc8fce2edd77d63d567220e36aee7b664b18f15fee50a17c4ee358a3283
api_id: Decimal::approx_root_with_prec
---
# Decimal::approx_root_with_prec

Compute a positive-degree root. Returns Result<Decimal, DecimalError> in Rust and Decimal or a structured exception in the bridge. degree: i64, transported as bigint; degree must be positive, otherwise InvalidRoot. Odd degrees accept negative input; even roots of negative values return NegativeRoot. Degree one is half-even rescaling. prec: isize, transported as an integer number; selects decimal places. Negative precision returns InvalidPrecision before domain checks. Rounding is half-even; exact midpoint roots such as sqrt(6.25) at precision zero round to 2. Despite the approx_root name, Rust certifies rounding using integer comparisons; exhausted core calculation/iteration budgets return NoConvergence. Input is borrowed and unchanged; results, including default zero, are initialized.

```rust
fn approx_root_with_prec (& self , degree : i64 , prec : isize) -> Result < Self , DecimalError >
```

## Normal usage

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-approx-root-with-prec" locale="en-US" />

## Browser boundaries

The [resource policy](../resource-policy) caps output scale at 1000. Nontrivial roots have degree at most 128, with preflight for scaling, Newton powers and midpoint-certification intermediates. Zero, signed unit values and degree-one roots retain core fast paths but still obey output precision limits. Domain and negative-precision errors retain library behavior; resource errors differ from NoConvergence. Valid individual inputs do not imply every degree/precision combination fits the browser budget.

## Related methods

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
