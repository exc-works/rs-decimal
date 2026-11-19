---
locale: en-US
source_revision: 521743c60e4285630a1e202d92aebd0d0b9f4764a73c56a223f9ec0c5662a6d6
api_id: "validation::ValidationError"
---
# validation::ValidationError

Inspect the failed validation record.

```rust
pub struct ValidationError
```

## Parameters, return and normal usage

A failure record with public field, tag and parameter properties. ValidationRule.validate returns it on failure; validate_decimal returns a list. JavaScript exposes editable field records suitable for translation. They are not DecimalError categories or exception messages. Editing an error record does not change the rule or decimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror" locale="en-US" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
