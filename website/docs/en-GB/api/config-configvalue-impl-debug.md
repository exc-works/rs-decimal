---
locale: en-GB
source_revision: 5f4867a89a40d93f24e8c716766cacdffb3d62078bd63369d4dd03f9f561ebeb
translation_provenance: en-US-reviewed-reuse
api_id: "config::ConfigValue::impl Debug"
---
# config::ConfigValue::impl Debug

Use the native Debug implementation.

```rust
impl Debug for config::ConfigValue
```

## Parameters, return and normal usage

ConfigValue is a small typed enum. Clone/Copy retain borrowed references rather than deeply copying their target. Debug is diagnostic text, not a stable wire format. Native Rust trait syntax is tested separately and is not a browser method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-debug" locale="en-GB" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
