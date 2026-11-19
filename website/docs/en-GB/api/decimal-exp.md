---
locale: en-GB
source_revision: f662d4a18feb3987ba22a3e426a493d661feab0dd71696533c5ec29aefece86d
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::exp
---
# Decimal::exp

Compute the exponential e^x. No parameters. Default output precision is max(input decimal places, 20), not 20 significant digits. All real inputs are allowed in the core. exp(0) is initialized one; a sufficiently negative argument can correctly round to initialized zero at the requested precision. The core uses interval enclosures and accepts a result only when rounding is certified. Rounding is half-even, without a mode parameter; failure to certify within core budgets returns NoConvergence rather than an unchecked approximation. Inputs remain unchanged and successful results are initialized.

```rust
fn exp (& self) -> Result < Self , DecimalError >
```

## Normal usage

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-exp" locale="en-GB" />

## Related methods

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
