---
locale: en-US
source_revision: 8c45e8f4230346a7c6fcd3cbb920d429b7a02acbcd8eb534c1f096fa8233828d
api_id: Decimal::add
---
# Decimal::add

Add decimals exactly.

```rust
fn add (& self , other : & Self) -> Self
```

## Parameters, return and normal usage

other: &Decimal; returns a new Decimal at the larger operand scale. Alignment is exact and does not discard digits. The Rust method is infallible; browser allocation limits can still return a resource error.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add" locale="en-US" />

## Browser boundaries

Inputs are borrowed and remain usable after success or ordinary errors. Returned Decimals own independent data; arithmetic results are initialized. The browser checks the [resource policy](../resource-policy) before calling the core: alignment, intermediate products, division scaling and output precision. Conservative preflight may reject calculations with small final results. Resource errors remain distinct from numerical-library errors; a panic terminates the run.

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
