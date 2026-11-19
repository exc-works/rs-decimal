---
locale: en-US
source_revision: 0bbd6c78da40405c6e9ad9c68a14878f98d98b4601eb8ff1296abba0bbf809ae
api_id: Decimal::sqrt
---
# Decimal::sqrt

Compute a square root. Returns Result<Decimal, DecimalError> in Rust and Decimal or a structured exception in the bridge. No degree parameter. Negative input returns NegativeRoot. Default precision is max(input decimal places, 20), not 20 significant digits. Rounding is half-even; exact midpoint roots such as sqrt(6.25) at precision zero round to 2. Despite the approx_root name, Rust certifies rounding using integer comparisons; exhausted core calculation/iteration budgets return NoConvergence. Input is borrowed and unchanged; results, including default zero, are initialized.

```rust
fn sqrt (& self) -> Result < Self , DecimalError >
```

## Normal usage

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-sqrt" locale="en-US" />

## Related methods

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
