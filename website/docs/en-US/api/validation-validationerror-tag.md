---
locale: en-US
source_revision: 0b44060089288355d3eba23edfcc95ed4174aef13953ff7dc6b74df01f566b11
api_id: "validation::ValidationError.tag"
---
# validation::ValidationError.tag

Inspect the failed validation record.

```rust
tag: & 'static str
```

## Parameters, return and normal usage

Public Rust static string identifying the failed rule. The browser uses an owned string property; unknown tags use the core default "{0} is invalid" template when translated. Editing an error record does not change the rule or decimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-tag" locale="en-US" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
