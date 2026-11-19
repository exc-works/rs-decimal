---
locale: en-US
source_revision: c31182c59d69a63bcf444ed429e3135dcff513f7463525a159b980c0e11afd3d
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

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
