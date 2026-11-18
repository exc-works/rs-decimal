---
locale: en-GB
source_revision: 9142df8b7aad31accc1292d48cdb5af145b94888294db67afaa241b5146ec0d5
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::sqrt_with_prec
---
# Decimal::sqrt_with_prec

Compute a square root. Returns Result<Decimal, DecimalError> in Rust and Decimal or a structured exception in the bridge. No degree parameter. Negative input returns NegativeRoot. prec: isize, transported as an integer number; selects decimal places. Negative precision returns InvalidPrecision before domain checks. Rounding is half-even; exact midpoint roots such as sqrt(6.25) at precision zero round to 2. Despite the approx_root name, Rust certifies rounding using integer comparisons; exhausted core calculation/iteration budgets return NoConvergence. Input is borrowed and unchanged; results, including default zero, are initialized.

```rust
fn sqrt_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Normal usage

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-sqrt-with-prec" locale="en-GB" />

## Browser boundaries

The [resource policy](../resource-policy) caps output scale at 1000. Nontrivial roots have degree at most 128, with preflight for scaling, Newton powers and midpoint-certification intermediates. Zero, signed unit values and degree-one roots retain core fast paths but still obey output precision limits. Domain and negative-precision errors retain library behavior; resource errors differ from NoConvergence. Valid individual inputs do not imply every degree/precision combination fits the browser budget.

## Related methods

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
