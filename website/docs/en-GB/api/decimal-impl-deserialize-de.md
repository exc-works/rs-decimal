---
locale: en-GB
source_revision: f2e44024c5b91fa0d6b53bc06008f77209b308fe37fd208a3ffa67a94ede4eb1
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::impl Deserialize < 'de >"
---
# Decimal::impl Deserialize < 'de >

Deserialize using the generic Serde visitor.

```rust
impl< 'de > Deserialize < 'de > for Decimal
```

## Parameters, return and normal usage

deserializer: D implementing Deserializer; returns Self or D::Error. Accepts exact decimal strings, integer visitor inputs and null. Floating-point visitor inputs are rejected: a JSON fractional/exponent number must use the dedicated raw JSON/config adapter instead. JSON null yields uninitialized Decimal zero. Empty strings are invalid. The browser static deserialize(source) adapter runs serde_json on complete original JSON text; it is narrower than arbitrary Rust deserializer support. Errors expose category="serde" and the original message.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-deserialize-de" locale="en-GB" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
