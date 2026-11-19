---
locale: en-US
source_revision: 5a5d502f0868797711c2e17ee061bb453dff465719822a493b3a33d226cf87b4
api_id: "config::ConfigValue::Nullable"
---
# config::ConfigValue::Nullable

Select Nullable configuration input.

```rust
Nullable (& 'a NullDecimal)
```

## Parameters, return and normal usage

Borrow an existing nullable. decode_nullable clones validity and hidden data; decode_decimal accepts only valid values and clones their inner Decimal. Later changes before decoding are observed.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-nullable" locale="en-US" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
