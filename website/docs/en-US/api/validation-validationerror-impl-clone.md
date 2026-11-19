---
locale: en-US
source_revision: 262ffddf12b7585e3d9a91337def5d4995343306864bd291c3fadc2686ec5701
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

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
