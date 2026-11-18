---
locale: en-US
source_revision: b6da66eb1330fd7e72548ba45454893fec7b7bdb425b5a72d19b09887a6620cd
api_id: "config::ConfigValue::Unsigned"
---
# config::ConfigValue::Unsigned

Select Unsigned configuration input.

```rust
Unsigned (u128)
```

## Parameters, return and normal usage

Carry an exact unsigned u128. The browser requires bigint in [0, 2^128-1], with no floating-point intermediary.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsigned" locale="en-US" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
