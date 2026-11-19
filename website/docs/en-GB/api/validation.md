---
locale: en-GB
source_revision: 668ba6d01bb4e8f8fc531b677bb3554fa942e3fa2858410dfc728efffb55ae7f
translation_provenance: en-US-reviewed-reuse
api_id: "validation"
---
# validation

Collect ordered rule failures and translate them.

```rust
pub mod validation
```

## Parameters, return and normal usage

Rust requires the validation feature, enabled in the browser build. The module groups exact rule evaluation and owned message templates without a framework or global registry. In the browser, use the validation namespace. Field names are caller text; translation substitutes them literally. Use required explicitly when missing input must fail.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation" locale="en-GB" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
