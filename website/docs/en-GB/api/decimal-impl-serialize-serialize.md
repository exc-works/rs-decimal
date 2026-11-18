---
locale: en-GB
source_revision: cc361ad62f1fb6d191828c27bb20bc087e8e30dee2f27936c25d2179d5adc40a
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="config-serde-decimal-impl-serialize-serialize" locale="en-GB" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
