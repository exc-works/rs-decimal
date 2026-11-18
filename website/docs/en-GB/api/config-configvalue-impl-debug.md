---
locale: en-GB
source_revision: 76e0ede0f8f2ecf84ebf0544bf21902acebb1483558125958da665053c84f11f
translation_provenance: en-US-reviewed-reuse
api_id: "config::ConfigValue::impl Debug"
---
# config::ConfigValue::impl Debug

Use the native Debug implementation.

```rust
impl Debug for config::ConfigValue
```

## Parameters, return and normal usage

ConfigValue is a small typed enum. Clone/Copy retain borrowed references rather than deeply copying their target. Debug is diagnostic text, not a stable wire format. Native Rust trait syntax is tested separately and is not a browser method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-debug" locale="en-GB" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
