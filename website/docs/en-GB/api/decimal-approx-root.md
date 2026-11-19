---
locale: en-GB
source_revision: 2e472bd0598b8ec796fb5adb09d98ce669a8d1defb11e5244f124c816e8d2cd8
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::approx_root
---
# Decimal::approx_root

Compute a positive-degree root. Returns Result<Decimal, DecimalError> in Rust and Decimal or a structured exception in the bridge. degree: i64, transported as bigint; degree must be positive, otherwise InvalidRoot. Odd degrees accept negative input; even roots of negative values return NegativeRoot. Degree one is half-even rescaling. Default precision is max(input decimal places, 20), not 20 significant digits. Rounding is half-even; exact midpoint roots such as sqrt(6.25) at precision zero round to 2. Despite the approx_root name, Rust certifies rounding using integer comparisons; exhausted core calculation/iteration budgets return NoConvergence. Input is borrowed and unchanged; results, including default zero, are initialized.

```rust
fn approx_root (& self , degree : i64) -> Result < Self , DecimalError >
```

## Normal usage

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-approx-root" locale="en-GB" />

## Related methods

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
