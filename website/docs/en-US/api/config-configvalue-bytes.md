---
locale: en-US
source_revision: 3576d2b52e6de01c1594ed23748d6a482f290fdb6f5b45fc15ec07a4cf297d34
api_id: "config::ConfigValue::Bytes"
---
# config::ConfigValue::Bytes

Select Bytes configuration input.

```rust
Bytes (& 'a [u8])
```

## Parameters, return and normal usage

Borrow bytes containing decimal text, not a binary decimal payload. Browser Uint8Array edits before decode are observed; decoding never modifies the array. Invalid UTF-8 fails decimal parsing.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-bytes" locale="en-US" />

## Browser boundaries

The config and serde features are enabled. Source text/bytes are capped at 8192 UTF-8 bytes; Rust checks decimal text and decoded JSON strings before parsing, so escapes cannot bypass the [resource policy](../resource-policy). 128-bit integers require in-range bigint. Borrowed Decimal/NullDecimal handles must still be live when decoded; conversion creates independent values without consuming inputs. Generic Serde trait behavior is demonstrated through concrete JSON adapters.

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
