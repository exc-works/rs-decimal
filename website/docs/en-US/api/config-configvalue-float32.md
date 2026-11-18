---
locale: en-US
source_revision: d858bd2326ab43e0cc593d11fb421e19dddaa88d9dec2ce79a679a247e556178
api_id: "config::ConfigValue::Float32"
---
# config::ConfigValue::Float32

Select Float32 configuration input.

```rust
Float32 (f32)
```

## Parameters, return and normal usage

Carry binary32. Rust narrows the browser number to f32 first and uses core conversion. Nonfinite input or narrowing overflow returns Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float32" locale="en-US" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
