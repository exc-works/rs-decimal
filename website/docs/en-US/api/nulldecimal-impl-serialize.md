---
locale: en-US
source_revision: 52852655da8fa507f812b593c8e5ba40f3e2342d29c1bda28c254c4a87933fda
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

<MethodExample id="config-serde-nulldecimal-impl-serialize" locale="en-US" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
