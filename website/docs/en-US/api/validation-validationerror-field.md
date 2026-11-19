---
locale: en-US
source_revision: 356d7409ac746daa69113ece073c46b80c7b34815cbe6ddfc49fc1c80e686226
api_id: "validation::ValidationError.field"
---
# validation::ValidationError.field

Inspect the failed validation record.

```rust
field: String
```

## Parameters, return and normal usage

Public owned String/string holding the caller-supplied field name. It is neither a property lookup path nor an HTML fragment. Braces are inserted literally during translation. Editing an error record does not change the rule or decimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-field" locale="en-US" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
