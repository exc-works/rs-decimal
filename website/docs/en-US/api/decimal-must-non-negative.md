---
locale: en-US
source_revision: 6440a61a644ecdbf15d2da0254f2f46ec6a0bac106e6a427225c0f45a05d250f
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

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
