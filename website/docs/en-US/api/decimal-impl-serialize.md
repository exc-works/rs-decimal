---
locale: en-US
source_revision: fa036d4ed1d96137decee6ee5f67e9b5d0d5b6d6fed77430c428472e7a62d1ff
api_id: "Decimal::impl Serialize"
---
# Decimal::impl Serialize

Serialize through the Rust Serde trait.

```rust
impl Serialize for Decimal 
```

## Parameters, return and normal usage

serializer: S implementing Serializer; returns S::Ok/S::Error. The core serializes canonical decimal strings or null. Uninitialized Decimal serializes as null; initialized zero serializes as string "0". Fractional trailing zeros are removed. The browser serialize() adapter chooses serde_json and returns complete JSON text. It does not expose arbitrary Rust serializers.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-serialize" locale="en-US" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
