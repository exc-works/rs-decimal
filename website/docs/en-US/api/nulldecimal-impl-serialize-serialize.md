---
locale: en-US
source_revision: c7dd2a0f7387f7110e7a059f6333707931d18a8d6fce4c06641505cdcb037d58
api_id: "NullDecimal::impl Serialize::serialize"
---
# NullDecimal::impl Serialize::serialize

Serialize through the Rust Serde trait.

```rust
fn serialize < S : Serializer > (& self , s : S) -> Result < S :: Ok , S :: Error >
```

## Parameters, return and normal usage

serializer: S implementing Serializer; returns S::Ok/S::Error. The core serializes canonical decimal strings or null. Invalid nullable values serialize as null; valid values with an uninitialized inner zero serialize as string "0". Fractional trailing zeros are removed. The browser serialize() adapter chooses serde_json and returns complete JSON text. It does not expose arbitrary Rust serializers.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-serialize-serialize" locale="en-US" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
