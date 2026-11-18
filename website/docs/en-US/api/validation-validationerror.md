---
locale: en-US
source_revision: 377152e0483ddcbebcd02f161fe043c129efdc1f915e3eb70b399c28e3d3906b
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

## Browser boundaries

The validation feature is enabled. Each string is capped at 8192 UTF-8 bytes; rule/error lists and message maps at 64 entries; JSON transport at 65536 bytes. Decimal rule parameters follow the [resource policy](../resource-policy). Translation preflights a conservative total substituted-text bound of 65536 bytes and may reject a just-fitting result. Custom tags and fields are data, never executed code. Validation failures, configuration errors and bridge resource errors remain distinct.

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
