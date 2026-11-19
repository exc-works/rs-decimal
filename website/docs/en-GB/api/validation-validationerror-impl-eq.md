---
locale: en-GB
source_revision: cfe29d83921c9d9fef3ddeed3842336ccc12799d1b0ed231fac999e4d87de7f7
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="validation-validation-validationerror-impl-eq" locale="en-GB" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
