---
locale: en-GB
source_revision: afcf4f720c241e6c6d4a096ef63464bb4e6b4cfb060705652f6b168cfc059cbe
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::safe_add
---
# Decimal::safe_add

Add decimals exactly and require a nonnegative result.

```rust
fn safe_add (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

other: &Decimal; returns a new Decimal at the larger operand scale. Alignment is exact and does not discard digits. Rust returns Result: only the result must be nonnegative, not each operand. A negative result returns InvalidArgument; zero is allowed.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-add" locale="en-GB" />

## Browser boundaries

Inputs are borrowed and remain usable after success or ordinary errors. Returned Decimals own independent data; arithmetic results are initialized. The browser checks the [resource policy](../resource-policy) before calling the core: alignment, intermediate products, division scaling and output precision. Conservative preflight may reject calculations with small final results. Resource errors remain distinct from numerical-library errors; a panic terminates the run.

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
