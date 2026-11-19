---
locale: en-US
source_revision: e064dade2acfdfd8dc8224a07d00162d5bdf94c66ca955bda753d76d5540ec7b
api_id: "config::ConfigValue::Float32"
---
# config::ConfigValue::Float32

Select Float32 configuration input.

```rust
Float32 (f32)
```

## Parameters, return and normal usage

Carry binary32. Rust narrows the browser number to f32 first and uses core conversion. Nonfinite input or narrowing overflow returns Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float32" locale="en-US" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
