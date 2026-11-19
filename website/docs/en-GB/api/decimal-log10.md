---
locale: en-GB
source_revision: ea6f97f51071f45a8c9bf2f4df22faacd07f9c9b025ed9b3ebf43ae9fab4e095
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::log10
---
# Decimal::log10

Compute the base-ten logarithm. No parameters. Default output precision is max(input decimal places, 20), not 20 significant digits. Input must be positive; zero and negative inputs return InvalidLog. Exact powers of the base have exact integer logarithms, retained at the output scale. The core uses interval enclosures and accepts a result only when rounding is certified. Rounding is half-even, without a mode parameter; failure to certify within core budgets returns NoConvergence rather than an unchecked approximation. Inputs remain unchanged and successful results are initialized.

```rust
fn log10 (& self) -> Result < Self , DecimalError >
```

## Normal usage

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log10" locale="en-GB" />

## Related methods

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
