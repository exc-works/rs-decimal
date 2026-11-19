---
locale: en-GB
source_revision: bfcaf340b0c1c1671610ce3a1025921897c59afd9bdf91da4be0b86f5f4d7446
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="config-serde-config-configvalue-bytes" locale="en-GB" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
