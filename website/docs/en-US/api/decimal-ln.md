---
locale: en-US
source_revision: 1b4d2150673750e48078239ad7618480e7fc5bb95b241e76137fd9b50e2c070d
api_id: Decimal::ln
---
# Decimal::ln

Compute the natural logarithm. No parameters. Default output precision is max(input decimal places, 20), not 20 significant digits. Input must be positive; zero and negative inputs return InvalidLog. ln(1) is exactly zero. The core uses interval enclosures and accepts a result only when rounding is certified. Rounding is half-even, without a mode parameter; failure to certify within core budgets returns NoConvergence rather than an unchecked approximation. Inputs remain unchanged and successful results are initialized.

```rust
fn ln (& self) -> Result < Self , DecimalError >
```

## Normal usage

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-ln" locale="en-US" />

## Related methods

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
