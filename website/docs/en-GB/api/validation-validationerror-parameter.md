---
locale: en-GB
source_revision: 9d8774ce76acc603ce438d5ebe120fafe780695ef55b74ea55b1fbe183940981
translation_provenance: en-US-reviewed-reuse
api_id: "validation::ValidationError.parameter"
---
# validation::ValidationError.parameter

Inspect the failed validation record.

```rust
parameter: String
```

## Parameters, return and normal usage

Public owned String/string retaining the original rule parameter exactly, including whitespace. It is substituted once as {1}, without recursive template interpretation. Editing an error record does not change the rule or decimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-parameter" locale="en-GB" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
