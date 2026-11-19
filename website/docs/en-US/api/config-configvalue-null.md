---
locale: en-US
source_revision: 846797abc20f334aac55a6cc16f18858d3e8f7870cd49af2d7f236e4e0d093b3
api_id: "config::ConfigValue::Null"
---
# config::ConfigValue::Null

Select Null configuration input.

```rust
Null
```

## Parameters, return and normal usage

An absent configuration input. decode_nullable returns an invalid value; decode_decimal rejects it with Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-null" locale="en-US" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
