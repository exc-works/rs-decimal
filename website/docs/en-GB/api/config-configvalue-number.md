---
locale: en-GB
source_revision: 74e7f2ffd43f198c10e7ebd972920c487311d6c7ab45db8c6281425ea90f9c14
translation_provenance: en-US-reviewed-reuse
api_id: "config::ConfigValue::Number"
---
# config::ConfigValue::Number

Select Number configuration input.

```rust
Number (& 'a str)
```

## Parameters, return and normal usage

Carry numeric source text without converting it through a float. The core parses decimal syntax; this variant is not a complete JSON grammar validator. Empty Number is invalid even for nullable targets.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-number" locale="en-GB" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
