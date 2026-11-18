---
locale: en-US
source_revision: 77287e6c4550f824e46a4556e0aaf001ccf92f4e78984f337df5b480f5f48874
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

<MethodExample id="config-serde-config-decode-nullable" locale="en-US" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
