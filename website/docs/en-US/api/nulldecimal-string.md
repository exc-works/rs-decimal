---
locale: en-US
source_revision: 5f7a180e14ebabfd6785a42604765a6ece20be12513e46794eb1f0299f911006
api_id: "NullDecimal::string"
---
# NullDecimal::string

Read nullable canonical text.

```rust
fn string (& self) -> String
```

## Parameters, return and normal usage

No parameters; returns String/string. Valid values use the inner Decimal canonical text, removing fractional trailing zeros. Invalid values return the literal null, regardless of hidden inner data. This differs from marshal_text, which returns empty bytes for invalid values. Reading does not change validity or initialization.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-string" locale="en-US" />

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
