---
locale: en-US
source_revision: 4973633c1cdc3ddfb696d250ca871028b9835fa4187520f393245e4690f6a0c3
api_id: "validation::ValidationMessages::with_messages"
---
# validation::ValidationMessages::with_messages

Override English validation templates.

```rust
fn with_messages (messages : BTreeMap < String , String >) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

messages: `BTreeMap<String,String>`/a string-valued object; returns `Result<ValidationMessages, DecimalError>`/a message handle. Overrides only provided keys; all other rules keep English defaults. Allowed keys are the twelve decimal_* rules. Empty templates, unknown keys, unmatched braces and placeholders other than {0}/{1} return InvalidArgument. Repeated valid placeholders are allowed. Does not change any global or previously constructed templates.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-with-messages" locale="en-US" />

## Browser boundaries

The validation feature is enabled. Each string is capped at 8192 UTF-8 bytes; rule/error lists and message maps at 64 entries; JSON transport at 65536 bytes. Decimal rule parameters follow the [resource policy](../resource-policy). Translation preflights a conservative total substituted-text bound of 65536 bytes and may reject a just-fitting result. Custom tags and fields are data, never executed code. Validation failures, configuration errors and bridge resource errors remain distinct.

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
