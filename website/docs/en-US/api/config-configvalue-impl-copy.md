---
locale: en-US
source_revision: 39668a70027991d27730b01fd29253411ac796d1b8a82a794c00f60c687cf5f8
api_id: "config::ConfigValue::impl Copy"
---
# config::ConfigValue::impl Copy

Use the native Copy implementation.

```rust
impl Copy for config::ConfigValue
```

## Parameters, return and normal usage

ConfigValue is a small typed enum. Clone/Copy retain borrowed references rather than deeply copying their target. Debug is diagnostic text, not a stable wire format. Native Rust trait syntax is tested separately and is not a browser method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-copy" locale="en-US" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
