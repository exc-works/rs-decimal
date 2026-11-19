---
locale: en-GB
source_revision: db21c0833b97369db1efcc9a73b34501c5dcac820064e9658bbb1cb161d8dec2
translation_provenance: en-US-reviewed-reuse
api_id: "validation::default_validation_messages"
---
# validation::default_validation_messages

Get a fresh map of locale templates.

```rust
fn default_validation_messages (locale : & str) -> BTreeMap < String , String >
```

## Parameters, return and normal usage

locale: &str/string; returns `BTreeMap<String,String>`/an ordinary object containing all twelve rule templates. Unknown locales use English. Each call owns its map, so edits do not affect later calls or other ValidationMessages objects. The reexport is the public module import path for the same function.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-default-validation-messages" locale="en-GB" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
