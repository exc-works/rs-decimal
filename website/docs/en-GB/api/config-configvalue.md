---
locale: en-GB
source_revision: fe8961def1f2d55b79394ddbdd152dccfbb21eaad02a1ae36eaa6898f24726be
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

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
