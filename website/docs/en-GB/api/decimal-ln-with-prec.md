---
locale: en-GB
source_revision: fbc64b05e8340067582332c66c6f66973b00c5a17412c42b00f322b92b77d915
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::ln_with_prec
---
# Decimal::ln_with_prec

Compute the natural logarithm. prec: nonnegative isize, represented by an integer number, selects decimal places; negative precision returns InvalidPrecision before domain checks. Input must be positive; zero and negative inputs return InvalidLog. ln(1) is exactly zero. The core uses interval enclosures and accepts a result only when rounding is certified. Rounding is half-even, without a mode parameter; failure to certify within core budgets returns NoConvergence rather than an unchecked approximation. Inputs remain unchanged and successful results are initialized.

```rust
fn ln_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Normal usage

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-ln-with-prec" locale="en-GB" />

## Related methods

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
