---
locale: en-US
source_revision: 9d0e7a5134f00bf353febb8beb1a65cb4dacb67d9c7e5b41dbb343b7baa674b0
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

## Browser boundaries

The validation feature is enabled. Each string is capped at 8192 UTF-8 bytes; rule/error lists and message maps at 64 entries; JSON transport at 65536 bytes. Decimal rule parameters follow the [resource policy](../resource-policy). Translation preflights a conservative total substituted-text bound of 65536 bytes and may reject a just-fitting result. Custom tags and fields are data, never executed code. Validation failures, configuration errors and bridge resource errors remain distinct.

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
