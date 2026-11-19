---
locale: en-GB
source_revision: 032d932903a81f0f03e2bde5c0ffb35c8a1064335f6ef0a3149d91077fa81075
translation_provenance: en-US-reviewed-reuse
api_id: "config::ConfigValue::Float64"
---
# config::ConfigValue::Float64

Select Float64 configuration input.

```rust
Float64 (f64)
```

## Parameters, return and normal usage

Carry binary64 through the core shortest decimal conversion. Nonfinite values return Unmarshal; digits already lost to floating-point rounding cannot be recovered.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float64" locale="en-GB" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
