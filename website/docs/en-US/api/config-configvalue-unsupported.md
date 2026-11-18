---
locale: en-US
source_revision: 2c10c2e73488c11940444f40ef8c96bd7f288472b5195e9bdeb57320696a7e16
api_id: "config::ConfigValue::Unsupported"
---
# config::ConfigValue::Unsupported

Select Unsupported configuration input.

```rust
Unsupported
```

## Parameters, return and normal usage

Represent an unsupported configuration type without coercion. Both decode functions return Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsupported" locale="en-US" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
