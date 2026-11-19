---
locale: en-US
source_revision: 7f9e2f6ed775898aae666efe4610ba891f54aadb6987e2a347de88c3c59500ea
api_id: Decimal::sqrt
---
# Decimal::sqrt

Compute a square root. No degree parameter. Negative input returns NegativeRoot. Default precision is max(input decimal places, 20), not 20 significant digits. Rounding is half-even; exact midpoint roots such as sqrt(6.25) at precision zero round to 2. Despite the approx_root name, Rust certifies rounding using integer comparisons; exhausted core calculation/iteration budgets return NoConvergence. Input is borrowed and unchanged; results, including default zero, are initialized.

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
