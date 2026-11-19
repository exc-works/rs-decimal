---
locale: en-US
source_revision: 572356860c801a3a257544cc180e1dca7bec96afecfd793ab3dbbcce432242dd
api_id: "validation::ValidationMessages::impl Clone"
---
# validation::ValidationMessages::impl Clone

Use the Rust Clone implementation.

```rust
impl Clone for validation::ValidationMessages
```

## Parameters, return and normal usage

Clone copies the complete rule, template map or failure fields. The copy is owned independently. This is native Rust trait syntax, not a browser method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-impl-clone" locale="en-US" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
