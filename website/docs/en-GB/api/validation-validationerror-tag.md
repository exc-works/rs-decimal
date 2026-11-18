---
locale: en-GB
source_revision: 6fae0dc88a563ce1dfaf67cbb1ece08366df9aad8969e9320dabf2cf06fa5933
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="validation-validation-validationerror-tag" locale="en-GB" />

## Browser boundaries

The validation feature is enabled. Each string is capped at 8192 UTF-8 bytes; rule/error lists and message maps at 64 entries; JSON transport at 65536 bytes. Decimal rule parameters follow the [resource policy](../resource-policy). Translation preflights a conservative total substituted-text bound of 65536 bytes and may reject a just-fitting result. Custom tags and fields are data, never executed code. Validation failures, configuration errors and bridge resource errors remain distinct.

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
