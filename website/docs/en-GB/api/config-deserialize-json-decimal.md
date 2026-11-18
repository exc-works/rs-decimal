---
locale: en-GB
source_revision: a26edb51e2fbe0ae6bac5abd8ce3e12bd13315bb3e836a0d554d5f19af0430ce
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

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
