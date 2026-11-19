---
locale: en-US
source_revision: 3fdc16376a6b5df85a2ce7cc24f6688fcdf41771e793df75d13c0d1b90a448b1
api_id: Decimal::sub_raw
---
# Decimal::sub_raw

Change the unscaled coefficient by an i64 integer.

```rust
fn sub_raw (& self , value : i64) -> Self
```

## Parameters, return and normal usage

value: i64, represented by bigint in JavaScript. Scale is preserved: at scale 2, adding/subtracting 1 changes the number by 0.01, not by 1. Rust returns a new Decimal. Numbers and out-of-i64 bigint values are rejected in JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub-raw" locale="en-US" />

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
