---
locale: en-US
source_revision: 4824e4dc634e3ef762c96aa330a4aec14949a376f5c5d2b586754496c1c0639f
api_id: "NullDecimal::impl Deserialize < 'de >::deserialize"
---
# NullDecimal::impl Deserialize < 'de >::deserialize

Deserialize using the generic Serde visitor.

```rust
fn deserialize < D : Deserializer < 'de > > (d : D) -> Result < Self , D :: Error >
```

## Parameters, return and normal usage

deserializer: D implementing Deserializer; returns Self or D::Error. Accepts exact decimal strings, integer visitor inputs and null. Floating-point visitor inputs are rejected: a JSON fractional/exponent number must use the dedicated raw JSON/config adapter instead. JSON null yields an invalid nullable with uninitialized inner zero. Empty strings are invalid. The browser static deserialize(source) adapter runs serde_json on complete original JSON text; it is narrower than arbitrary Rust deserializer support. Errors expose category="serde" and the original message.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-deserialize-de-deserialize" locale="en-US" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
