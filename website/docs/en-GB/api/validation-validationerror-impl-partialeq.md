---
locale: en-GB
source_revision: 88d2cfb170b637934d794950f5a609aa140f9cabce333b9eba44daa4d52bc8fc
translation_provenance: en-US-reviewed-reuse
api_id: "validation::ValidationError::impl PartialEq"
---
# validation::ValidationError::impl PartialEq

Use the Rust PartialEq implementation.

```rust
impl PartialEq for validation::ValidationError
```

## Parameters, return and normal usage

ValidationError equality compares field, tag and original parameter together; it does not just compare translated messages. This is native Rust trait syntax, not a browser method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-partialeq" locale="en-GB" />

## Browser boundaries

The validation feature is enabled. Each string is capped at 8192 UTF-8 bytes; rule/error lists and message maps at 64 entries; JSON transport at 65536 bytes. Decimal rule parameters follow the [resource policy](../resource-policy). Translation preflights a conservative total substituted-text bound of 65536 bytes and may reject a just-fitting result. Custom tags and fields are data, never executed code. Validation failures, configuration errors and bridge resource errors remain distinct.

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
