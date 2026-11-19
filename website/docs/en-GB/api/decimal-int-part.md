---
locale: en-GB
source_revision: 0b7e4ede82a7865b24f3ea40debe791505bf9ac255c8e2f8790aa1271d0fdf3a
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::int_part
---
# Decimal::int_part

Extract the integer part by truncating toward zero.

```rust
fn int_part (& self) -> BigInt
```

## Parameters, return and normal usage

No parameters; Rust returns BigInt and JavaScript returns bigint, without floating-point conversion. Negative -12.75 yields -12, not floor(-12.75) = -13. Default zero yields zero. The example serializes the integer as a string to preserve arbitrary precision in JSON.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-int-part" locale="en-GB" />

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
