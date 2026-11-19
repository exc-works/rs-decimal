---
locale: en-GB
source_revision: 3594148ee83ab0bf68ef800a2a73612702e5a4d6ff495df20f526a7cdddcfcf1
translation_provenance: en-US-reviewed-reuse
api_id: "config::deserialize_json_decimal"
---
# config::deserialize_json_decimal

Deserialize an exact JSON configuration field.

```rust
fn deserialize_json_decimal < 'de , D : Deserializer < 'de > > (deserializer : D ,) -> Result < Decimal , D :: Error >
```

## Parameters, return and normal usage

deserializer: D implementing Serde Deserializer; intended for #[serde(deserialize_with = "rs_decimal::config::deserialize_json_decimal")]. Requires a serde_json source and config feature. Retains original numeric tokens without a float round trip. The browser provides a concrete adapter taking one complete JSON scalar source string, not an arbitrary deserializer. JSON null and empty strings are rejected for the non-nullable target. Errors have the Serde deserializer error type; browser category="serde" preserves its message rather than inventing DecimalError kinds. Trailing non-whitespace input is rejected.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-deserialize-json-decimal" locale="en-GB" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
