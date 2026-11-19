---
locale: en-GB
source_revision: b55dba0922642cda4e225685c47d97a54da01243942c47cb892b8c9136202f1a
translation_provenance: en-US-reviewed-reuse
api_id: "validation::ValidationRule::impl Clone"
---
# validation::ValidationRule::impl Clone

Use the Rust Clone implementation.

```rust
impl Clone for validation::ValidationRule
```

## Parameters, return and normal usage

Clone copies the complete rule, template map or failure fields. The copy is owned independently. This is native Rust trait syntax, not a browser method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-impl-clone" locale="en-GB" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
