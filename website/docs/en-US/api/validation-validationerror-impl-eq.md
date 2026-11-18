---
locale: en-US
source_revision: 4b5fc87cb7547a8cb739a71f9c75b83ad78ac1a07faee17459dcd903e68f40c4
api_id: "validation::ValidationError::impl Eq"
---
# validation::ValidationError::impl Eq

Use the Rust Eq implementation.

```rust
impl Eq for validation::ValidationError
```

## Parameters, return and normal usage

ValidationError equality compares field, tag and original parameter together; it does not just compare translated messages. This is native Rust trait syntax, not a browser method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-eq" locale="en-US" />

## Browser boundaries

The validation feature is enabled. Each string is capped at 8192 UTF-8 bytes; rule/error lists and message maps at 64 entries; JSON transport at 65536 bytes. Decimal rule parameters follow the [resource policy](../resource-policy). Translation preflights a conservative total substituted-text bound of 65536 bytes and may reject a just-fitting result. Custom tags and fields are data, never executed code. Validation failures, configuration errors and bridge resource errors remain distinct.

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
