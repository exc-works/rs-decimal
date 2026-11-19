---
locale: en-US
source_revision: 71b55f4d77a06034a780155a4f624e277f9df950e286fd15014d7cc9e03ee581
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

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
