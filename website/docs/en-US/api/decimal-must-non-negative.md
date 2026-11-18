---
locale: en-US
source_revision: 9c252d1ff038e1dac4451e4dd728020f00ea8f88567265b756b2f63f4474b26e
api_id: Decimal::must_non_negative
---
# Decimal::must_non_negative

Require nonnegativity and return an initialized copy.

```rust
fn must_non_negative (& self) -> Self
```

## Parameters, return and normal usage

No parameters; returns Decimal for nonnegative input, including zero. A negative input panics with a string payload in Rust. In Wasm this is a trap, not a recoverable DecimalError; it ends the playground run. Start a new run for fresh Wasm state. This is a numeric assertion, not an absolute-value conversion; use abs to change a sign.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-must-non-negative" locale="en-US" />

## Browser boundaries

Inputs are borrowed and remain usable after success or ordinary errors. Returned Decimals own independent data; arithmetic results are initialized. The browser checks the [resource policy](../resource-policy) before calling the core: alignment, intermediate products, division scaling and output precision. Conservative preflight may reject calculations with small final results. Resource errors remain distinct from numerical-library errors; a panic terminates the run.

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
