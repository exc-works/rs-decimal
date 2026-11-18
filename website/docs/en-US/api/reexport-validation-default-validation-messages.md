---
locale: en-US
source_revision: 96429a6b772c7e5f91528e442f9be3179ac231b38327e61764d5f24018444428
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

<MethodExample id="validation-reexport-validation-default-validation-messages" locale="en-US" />

## Browser boundaries

The validation feature is enabled. Each string is capped at 8192 UTF-8 bytes; rule/error lists and message maps at 64 entries; JSON transport at 65536 bytes. Decimal rule parameters follow the [resource policy](../resource-policy). Translation preflights a conservative total substituted-text bound of 65536 bytes and may reject a just-fitting result. Custom tags and fields are data, never executed code. Validation failures, configuration errors and bridge resource errors remain distinct.

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
