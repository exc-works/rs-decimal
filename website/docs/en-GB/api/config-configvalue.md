---
locale: en-GB
source_revision: 76c1141f3b7421518d2bc3a9f102aa654b735f07d57812e6208ffd1b784510f2
translation_provenance: en-US-reviewed-reuse
api_id: "config::ConfigValue"
---
# config::ConfigValue

Decode explicitly typed configuration values.

```rust
pub enum ConfigValue< 'a >
```

## Parameters, return and normal usage

The typed configuration adapter supports text/byte text, exact numeric text, signed/unsigned 128-bit integers, floats and borrowed Decimal/NullDecimal values. Rust requires config. Non-nullable decoding rejects Null and invalid Nullable rather than inventing zero. Existing Decimal and valid Nullable values are cloned with full representation, including uninitialized zero. Text/Number/Bytes parse exact decimal text without stripping SQL quotes. Floats already carry binary rounding; use text or bigint to retain exact source digits. Browser variants belong to one run and borrow handles/buffers until decoding.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue" locale="en-GB" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
