---
locale: en-US
source_revision: 14996308295f3d9761be44a618304820607fea78ed2b85488ce4a81975821f38
api_id: Decimal::ln_with_prec
---
# Decimal::ln_with_prec

Compute the natural logarithm, returning Result<Decimal, DecimalError> in Rust or a Decimal/structured exception in the bridge. prec: nonnegative isize, represented by an integer number, selects decimal places; negative precision returns InvalidPrecision before domain checks. Input must be positive; zero and negative inputs return InvalidLog. ln(1) is exactly zero. The core uses interval enclosures and accepts a result only when rounding is certified. Rounding is half-even, without a mode parameter; failure to certify within core budgets returns NoConvergence rather than an unchecked approximation. Inputs remain unchanged and successful results are initialized.

```rust
fn ln_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Normal usage

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-ln-with-prec" locale="en-US" />

## Related methods

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
