---
locale: en-US
source_revision: 07611a32fa9c50a924b808d879544951b6661718e9f82991f25c28bfa7a925e8
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

<MethodExample id="config-serde-config-configvalue-signed" locale="en-US" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
