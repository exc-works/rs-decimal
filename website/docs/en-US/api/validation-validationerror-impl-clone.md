---
locale: en-US
source_revision: d543ee5c1ea405eaf1deafc27267bde1bfdd8a1593aa6059be6405a03110ee1f
api_id: "validation::ValidationError::impl Clone"
---
# validation::ValidationError::impl Clone

Use the Rust Clone implementation.

```rust
impl Clone for validation::ValidationError
```

## Parameters, return and normal usage

Clone copies the complete rule, template map or failure fields. The copy is owned independently. This is native Rust trait syntax, not a browser method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-clone" locale="en-US" />

## Browser boundaries

The validation feature is enabled. Each string is capped at 8192 UTF-8 bytes; rule/error lists and message maps at 64 entries; JSON transport at 65536 bytes. Decimal rule parameters follow the [resource policy](../resource-policy). Translation preflights a conservative total substituted-text bound of 65536 bytes and may reject a just-fitting result. Custom tags and fields are data, never executed code. Validation failures, configuration errors and bridge resource errors remain distinct.

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
