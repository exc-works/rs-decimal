---
locale: en-US
source_revision: 03da42d48c5d8b250daaaa93d59162466d4ca8b88da11e32c7b577b2d36d0747
api_id: "config"
---
# config

Decode explicitly typed configuration values.

```rust
pub mod config
```

## Parameters, return and normal usage

The typed configuration adapter supports text/byte text, exact numeric text, signed/unsigned 128-bit integers, floats and borrowed Decimal/NullDecimal values. Rust requires config. Non-nullable decoding rejects Null and invalid Nullable rather than inventing zero. Existing Decimal and valid Nullable values are cloned with full representation, including uninitialized zero. Text/Number/Bytes parse exact decimal text without stripping SQL quotes. Floats already carry binary rounding; use text or bigint to retain exact source digits. Browser variants belong to one run and borrow handles/buffers until decoding.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config" locale="en-US" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
