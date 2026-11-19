---
locale: en-US
source_revision: c983f6ed089705857a773a0bcfbea0bf32771eb2c76c31e24f0160ce317c8980
api_id: Decimal::add_raw
---
# Decimal::add_raw

Change the unscaled coefficient by an i64 integer.

```rust
fn add_raw (& self , value : i64) -> Self
```

## Parameters, return and normal usage

value: i64, represented by bigint in the bridge. Scale is preserved: at scale 2, adding/subtracting 1 changes the number by 0.01, not by 1. Rust returns a new Decimal. Numbers and out-of-i64 bigint values are rejected by the bridge.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add-raw" locale="en-US" />

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
