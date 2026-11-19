---
locale: en-GB
source_revision: 31042d691902e2dc4bdadd92d4688638c315fb08be40c79d11b031b854afc6bc
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::sub_raw
---
# Decimal::sub_raw

Change the unscaled coefficient by an i64 integer.

```rust
fn sub_raw (& self , value : i64) -> Self
```

## Parameters, return and normal usage

value: i64, represented by bigint in the bridge. Scale is preserved: at scale 2, adding/subtracting 1 changes the number by 0.01, not by 1. Rust returns a new Decimal. Numbers and out-of-i64 bigint values are rejected by the bridge.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub-raw" locale="en-GB" />

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
