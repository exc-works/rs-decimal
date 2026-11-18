---
locale: en-GB
source_revision: 5bb3ef8c2d718846f6ac7a4004c6720c37095a64615189ff33906f2fba479837
translation_provenance: en-US-reviewed-reuse
api_id: "config::ConfigValue::Signed"
---
# config::ConfigValue::Signed

Select Signed configuration input.

```rust
Signed (i128)
```

## Parameters, return and normal usage

Carry an exact signed i128. The browser requires bigint in [-2^127, 2^127-1]; JS numbers and out-of-range values are rejected before conversion.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-signed" locale="en-GB" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
