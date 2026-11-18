---
locale: en-US
source_revision: 4812fffbe74c100e5632736fe3ca1c5dd726efa9358bb2ca70cc5db51e55bd14
api_id: Decimal::exp
---
# Decimal::exp

Compute the exponential e^x, returning Result<Decimal, DecimalError> in Rust or a Decimal/structured exception in the bridge. No parameters. Default output precision is max(input decimal places, 20), not 20 significant digits. All real inputs are allowed in the core. exp(0) is initialized one; a sufficiently negative argument can correctly round to initialized zero at the requested precision. The core uses interval enclosures and accepts a result only when rounding is certified. Rounding is half-even, without a mode parameter; failure to certify within core budgets returns NoConvergence rather than an unchecked approximation. Inputs remain unchanged and successful results are initialized.

```rust
fn exp (& self) -> Result < Self , DecimalError >
```

## Normal usage

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-exp" locale="en-US" />

## Browser boundaries

This family has additional [resource limits](../resource-policy): output precision at most 128, input scale at most 256 and input coefficient at most 4096 bits. Ordinary exp evaluation also requires absolute input at most 64; negative inputs that the core proves round directly to zero retain that fast path. Large input scale can make a no-argument method choose a default precision above 128; use with_prec to select an allowed output precision explicitly. Resource errors differ from core NoConvergence. Stop interrupts a run; the next run creates a fresh Wasm instance.

## Related methods

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
