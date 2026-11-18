---
locale: en-GB
source_revision: b283786af672117d0c3017e513f97f5df201a379427acab824b383812aa4cf93
translation_provenance: en-US-reviewed-reuse
api_id: "config::ConfigValue::Decimal"
---
# config::ConfigValue::Decimal

Select Decimal configuration input.

```rust
Decimal (& 'a Decimal)
```

## Parameters, return and normal usage

Borrow an existing decimal and clone coefficient, scale and initialization during decoding. The source stays usable. Disposing/moving a browser source before decoding invalidates the borrowed variant.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-decimal" locale="en-GB" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
