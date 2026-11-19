---
locale: en-GB
source_revision: 89abd084e8bb8541133194e2046ba4621ebe68fb1c342334ebc3bf5d1ca7dcec
translation_provenance: en-US-reviewed-reuse
api_id: "config::ConfigValue::Text"
---
# config::ConfigValue::Text

Select Text configuration input.

```rust
Text (& 'a str)
```

## Parameters, return and normal usage

Borrow exact decimal text. Surrounding whitespace is parsed by the decimal core; SQL-style outer quotes are not removed. Only exactly empty text has nullable-absence semantics.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-text" locale="en-GB" />

## Related methods

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
