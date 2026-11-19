---
locale: en-US
source_revision: 7350b5ad5bfd585c249094865cd49a6bad71c645ac5e1b3b72695308a1cc7d0a
api_id: Decimal::log10_with_prec
---
# Decimal::log10_with_prec

Compute the base-ten logarithm, returning Result<Decimal, DecimalError> in Rust or a Decimal/structured exception in the bridge. prec: nonnegative isize, represented by an integer number, selects decimal places; negative precision returns InvalidPrecision before domain checks. Input must be positive; zero and negative inputs return InvalidLog. Exact powers of the base have exact integer logarithms, retained at the output scale. The core uses interval enclosures and accepts a result only when rounding is certified. Rounding is half-even, without a mode parameter; failure to certify within core budgets returns NoConvergence rather than an unchecked approximation. Inputs remain unchanged and successful results are initialized.

```rust
fn log10_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Normal usage

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log10-with-prec" locale="en-US" />

## Related methods

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
