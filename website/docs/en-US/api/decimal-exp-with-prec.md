---
locale: en-US
source_revision: a4afb74edf390a8e75f7a3a242d431661b5b997a3026d6a0446a646f7a0d41bb
api_id: Decimal::exp_with_prec
---
# Decimal::exp_with_prec

Compute the exponential e^x. prec: nonnegative isize, represented by an integer number, selects decimal places; negative precision returns InvalidPrecision before domain checks. All real inputs are allowed in the core. exp(0) is initialized one; a sufficiently negative argument can correctly round to initialized zero at the requested precision. The core uses interval enclosures and accepts a result only when rounding is certified. Rounding is half-even, without a mode parameter; failure to certify within core budgets returns NoConvergence rather than an unchecked approximation. Inputs remain unchanged and successful results are initialized.

```rust
fn exp_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Normal usage

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-exp-with-prec" locale="en-US" />

## Related methods

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
