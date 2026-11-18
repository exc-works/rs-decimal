---
locale: en-GB
source_revision: 178894cd0af1970f19442cd816169372cdb2496adc78c87aaec82dd961b0da2e
translation_provenance: en-US-reviewed-reuse
api_id: "config::ConfigValue::impl Copy"
---
# config::ConfigValue::impl Copy

Use the native Copy implementation.

```rust
impl Copy for config::ConfigValue
```

## Parameters, return and normal usage

ConfigValue is a small typed enum. Clone/Copy retain borrowed references rather than deeply copying their target. Debug is diagnostic text, not a stable wire format. Native Rust trait syntax is tested separately and is not a browser method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-copy" locale="en-GB" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
