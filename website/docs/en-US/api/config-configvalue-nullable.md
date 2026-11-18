---
locale: en-US
source_revision: 4748c543f00fa16cc184b2f3ac3218ce12c09982f49453b64f19d87393fb722a
api_id: "config::ConfigValue::Nullable"
---
# config::ConfigValue::Nullable

Select Nullable configuration input.

```rust
Nullable (& 'a NullDecimal)
```

## Parameters, return and normal usage

Borrow an existing nullable. decode_nullable clones validity and hidden data; decode_decimal accepts only valid values and clones their inner Decimal. Later changes before decoding are observed.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-nullable" locale="en-US" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
