---
locale: en-GB
source_revision: 3ccd35e370bf1f1ede8c557955aa9396976fa4ff57cb089c5b7cc2f5b8ec4b48
translation_provenance: en-US-reviewed-reuse
api_id: "validation::ValidationRule::impl Debug"
---
# validation::ValidationRule::impl Debug

Use the Rust Debug implementation.

```rust
impl Debug for validation::ValidationRule
```

## Parameters, return and normal usage

Debug produces diagnostic text, not a stable serialized validation format. This is native Rust trait syntax, not a browser method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-impl-debug" locale="en-GB" />

## Browser boundaries

The validation feature is enabled. Each string is capped at 8192 UTF-8 bytes; rule/error lists and message maps at 64 entries; JSON transport at 65536 bytes. Decimal rule parameters follow the [resource policy](../resource-policy). Translation preflights a conservative total substituted-text bound of 65536 bytes and may reject a just-fitting result. Custom tags and fields are data, never executed code. Validation failures, configuration errors and bridge resource errors remain distinct.

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
