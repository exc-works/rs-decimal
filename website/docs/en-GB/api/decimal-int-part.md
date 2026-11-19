---
locale: en-GB
source_revision: b7ff1544694c51babb7af7cb99735ecc6687d86e9cd9a108af321bed93252beb
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::int_part
---
# Decimal::int_part

Extract the integer part by truncating toward zero.

```rust
fn int_part (& self) -> BigInt
```

## Parameters, return and normal usage

No parameters; Rust returns BigInt and the bridge returns bigint, without floating-point conversion. Negative -12.75 yields -12, not floor(-12.75) = -13. Default zero yields zero. The example serializes the integer as a string to preserve arbitrary precision in JSON.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-int-part" locale="en-GB" />

## Related methods

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
