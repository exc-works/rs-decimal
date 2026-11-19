---
locale: en-US
source_revision: ad5bc09e58ba54af3a20899560f38f4b2682294a4c7489043e5331092c1467a0
api_id: Decimal::add_raw
---
# Decimal::add_raw

Change the unscaled coefficient by an i64 integer.

```rust
fn add_raw (& self , value : i64) -> Self
```

## Parameters, return and normal usage

value: i64, represented by bigint in JavaScript. Scale is preserved: at scale 2, adding/subtracting 1 changes the number by 0.01, not by 1. Rust returns a new Decimal. Numbers and out-of-i64 bigint values are rejected in JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add-raw" locale="en-US" />

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
