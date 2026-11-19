---
locale: en-US
source_revision: 8ffc267a7c40a9915ad1544633aa5847696df2f114f2a6275b83514da10320eb
api_id: "validation::ValidationRule::validate"
---
# validation::ValidationRule::validate

Validate one field against a rule.

```rust
fn validate (& self , field : & str , value : & Decimal) -> Result < () , ValidationError >
```

## Parameters, return and normal usage

field: &str/string, value: &Decimal/Decimal; returns `Result<(), ValidationError>`. Success is undefined in JavaScript. Failure throws a JS ValidationError with category="validation" and field/tag/parameter properties; it is distinct from DecimalError. Field and original parameter text remain unchanged, including braces or whitespace. The rule and value are borrowed. Use validate_decimal to collect multiple failures without throwing.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-validate" locale="en-US" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
