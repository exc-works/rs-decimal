---
locale: en-GB
source_revision: 324990894f6327d9f0e88937d867ea0f32bc3c792ba37210279bca0b73495f73
translation_provenance: en-US-reviewed-reuse
api_id: "config::decode_nullable"
---
# config::decode_nullable

Decode an optional configuration value.

```rust
fn decode_nullable (value : ConfigValue < '_ >) -> Result < NullDecimal , DecimalError >
```

## Parameters, return and normal usage

value: ConfigValue; returns `Result<NullDecimal, DecimalError>`/a new nullable handle. Null, exactly empty Text and empty Bytes become invalid with an uninitialized inner zero. Whitespace-only Text and empty Number still fail decimal parsing. Existing Nullable values clone both validity and hidden inner data, even when invalid; a borrowed Decimal becomes valid without forcing initialization. Other inputs follow decode_decimal and become valid on success. Inputs are not consumed.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-nullable" locale="en-GB" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
