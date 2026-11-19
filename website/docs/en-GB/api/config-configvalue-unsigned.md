---
locale: en-GB
source_revision: 9e6af4fbdd99e4ba366c644aaf4e403edbed7726d41f29fd990845c2a190132b
translation_provenance: en-US-reviewed-reuse
api_id: "config::ConfigValue::Unsigned"
---
# config::ConfigValue::Unsigned

Select Unsigned configuration input.

```rust
Unsigned (u128)
```

## Parameters, return and normal usage

Carry an exact unsigned u128. The browser requires bigint in [0, 2^128-1], with no floating-point intermediary.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsigned" locale="en-GB" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
