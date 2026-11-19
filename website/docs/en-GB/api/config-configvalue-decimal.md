---
locale: en-GB
source_revision: 8d101e011ba2e4f441ebfc6371c73cb22589c6224599cc87e367af949b20dc8f
translation_provenance: en-US-reviewed-reuse
api_id: "config::ConfigValue::Decimal"
---
# config::ConfigValue::Decimal

Select Decimal configuration input.

```rust
Decimal (& 'a Decimal)
```

## Parameters, return and normal usage

Borrow an existing decimal and clone coefficient, scale and initialization during decoding. The source stays usable. Disposing/moving a browser source before decoding invalidates the borrowed variant.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-decimal" locale="en-GB" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
