---
locale: en-GB
source_revision: db26ba148bd1b6fe560088f6be1bce313faff866656299f7c24eeb47568b46bf
translation_provenance: en-US-reviewed-reuse
api_id: "config::ConfigValue::impl Clone"
---
# config::ConfigValue::impl Clone

Use the native Clone implementation.

```rust
impl Clone for config::ConfigValue
```

## Parameters, return and normal usage

ConfigValue is a small typed enum. Clone/Copy retain borrowed references rather than deeply copying their target. Debug is diagnostic text, not a stable wire format. Native Rust trait syntax is tested separately and is not a browser method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-clone" locale="en-GB" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
