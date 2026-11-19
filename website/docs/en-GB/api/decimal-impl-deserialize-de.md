---
locale: en-GB
source_revision: 290824d81667633f4375180726adda2c851b5415686c46a9a13af6b626baaf4f
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

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
