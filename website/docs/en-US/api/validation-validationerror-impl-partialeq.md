---
locale: en-US
source_revision: 7f1c2832917badced6e78a9c287520234452b297109e8406a028c3703581c859
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

<MethodExample id="validation-validation-validationerror-impl-partialeq" locale="en-US" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
