---
locale: en-US
source_revision: 4a95a976d839a4b08d68e1a4e5b0d124596f396891cbc4d74282ca10c6649baa
api_id: Decimal::approx_root
---
# Decimal::approx_root

Compute a positive-degree root. degree: i64, transported as bigint; degree must be positive, otherwise InvalidRoot. Odd degrees accept negative input; even roots of negative values return NegativeRoot. Degree one is half-even rescaling. Default precision is max(input decimal places, 20), not 20 significant digits. Rounding is half-even; exact midpoint roots such as sqrt(6.25) at precision zero round to 2. Despite the approx_root name, Rust certifies rounding using integer comparisons; exhausted core calculation/iteration budgets return NoConvergence. Input is borrowed and unchanged; results, including default zero, are initialized.

```rust
fn approx_root (& self , degree : i64) -> Result < Self , DecimalError >
```

## Normal usage

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-approx-root" locale="en-US" />

## Related methods

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
