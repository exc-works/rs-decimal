---
locale: en-GB
source_revision: 48f0114735b7fdf69a527f5e2e6dcf6a17a2e69d41b0a53a9ff65fb0ac1362fd
translation_provenance: en-US-reviewed-reuse
api_id: "NullDecimal::impl Serialize"
---
# NullDecimal::impl Serialize

Serialize through the Rust Serde trait.

```rust
impl Serialize for NullDecimal
```

## Parameters, return and normal usage

serializer: S implementing Serializer; returns S::Ok/S::Error. The core serializes canonical decimal strings or null. Invalid nullable values serialize as null; valid values with an uninitialized inner zero serialize as string "0". Fractional trailing zeros are removed. The browser serialize() adapter chooses serde_json and returns complete JSON text. It does not expose arbitrary Rust serializers.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-serialize" locale="en-GB" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
