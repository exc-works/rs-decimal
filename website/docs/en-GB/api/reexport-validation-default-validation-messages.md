---
locale: en-GB
source_revision: c829b94ceed61e0a65772da94750a53ecb8269151bdb6f1efd6a7c407ad09f12
translation_provenance: en-US-reviewed-reuse
api_id: "reexport::validation::default_validation_messages"
---
# reexport::validation::default_validation_messages

Get a fresh map of locale templates.

```rust
pub use validation::messages::default_validation_messages as default_validation_messages
```

## Parameters, return and normal usage

locale: &str/string; returns `BTreeMap<String,String>`/an ordinary object containing all twelve rule templates. Unknown locales use English. Each call owns its map, so edits do not affect later calls or other ValidationMessages objects. The reexport is the public module import path for the same function.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-reexport-validation-default-validation-messages" locale="en-GB" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
