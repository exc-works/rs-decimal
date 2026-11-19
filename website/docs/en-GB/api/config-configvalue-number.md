---
locale: en-GB
source_revision: d9b1450ea64f3f424b8c3303a77d21fe7ac3c0fd6d9a478841f3ae7df9627834
translation_provenance: en-US-reviewed-reuse
api_id: "config::ConfigValue::Number"
---
# config::ConfigValue::Number

Select Number configuration input.

```rust
Number (& 'a str)
```

## Parameters, return and normal usage

Carry numeric source text without converting it through a float. The core parses decimal syntax; this variant is not a complete JSON grammar validator. Empty Number is invalid even for nullable targets.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-number" locale="en-GB" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
