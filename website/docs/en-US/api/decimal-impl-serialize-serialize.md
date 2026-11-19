---
locale: en-US
source_revision: b7aac8fb7a35df9ddf10adc50d88fb1bfd0169290aa22d5f4ad96531651b49c8
api_id: "Decimal::impl Serialize::serialize"
---
# Decimal::impl Serialize::serialize

Serialize through the Rust Serde trait.

```rust
fn serialize < S : Serializer > (& self , s : S) -> Result < S :: Ok , S :: Error >
```

## Parameters, return and normal usage

serializer: S implementing Serializer; returns S::Ok/S::Error. The core serializes canonical decimal strings or null. Uninitialized Decimal serializes as null; initialized zero serializes as string "0". Fractional trailing zeros are removed. The browser serialize() adapter chooses serde_json and returns complete JSON text. It does not expose arbitrary Rust serializers.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-serialize-serialize" locale="en-US" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
