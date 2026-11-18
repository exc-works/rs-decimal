---
locale: en-GB
source_revision: 9cc3411fb84577d5ea175fe7b4b5d7e0d13f86f3ba54a79c3a0728b721c31211
translation_provenance: en-US-reviewed-reuse
api_id: "config::ConfigValue::Text"
---
# config::ConfigValue::Text

Select Text configuration input.

```rust
Text (& 'a str)
```

## Parameters, return and normal usage

Borrow exact decimal text. Surrounding whitespace is parsed by the decimal core; SQL-style outer quotes are not removed. Only exactly empty text has nullable-absence semantics.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-text" locale="en-GB" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
