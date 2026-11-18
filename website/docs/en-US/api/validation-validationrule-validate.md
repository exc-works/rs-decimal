---
locale: en-US
source_revision: 92b27b2e44b2095f2d604aead4c4f01fb922094a0083fe1c3c5e2509eaa4b3d7
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

## Browser boundaries

The validation feature is enabled. Each string is capped at 8192 UTF-8 bytes; rule/error lists and message maps at 64 entries; JSON transport at 65536 bytes. Decimal rule parameters follow the [resource policy](../resource-policy). Translation preflights a conservative total substituted-text bound of 65536 bytes and may reject a just-fitting result. Custom tags and fields are data, never executed code. Validation failures, configuration errors and bridge resource errors remain distinct.

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
