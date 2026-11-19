---
locale: en-US
source_revision: 79f19f9f6f20dd385b69becd2b7e8e1abfeb536b960dc7d8b240d8301a6064de
api_id: "config::deserialize_json_nullable"
---
# config::deserialize_json_nullable

Deserialize an exact JSON configuration field.

```rust
fn deserialize_json_nullable < 'de , D : Deserializer < 'de > > (deserializer : D ,) -> Result < NullDecimal , D :: Error >
```

## Parameters, return and normal usage

deserializer: D implementing Serde Deserializer; intended for #[serde(deserialize_with = "rs_decimal::config::deserialize_json_nullable")]. Requires a serde_json source and config feature. Retains original numeric tokens without a float round trip. The browser provides a concrete adapter taking one complete JSON scalar source string, not an arbitrary deserializer. JSON null and exactly empty JSON string become invalid; whitespace-only strings still fail. Successful nonempty values become valid. Errors have the Serde deserializer error type; browser category="serde" preserves its message rather than inventing DecimalError kinds. Trailing non-whitespace input is rejected.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-deserialize-json-nullable" locale="en-US" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
