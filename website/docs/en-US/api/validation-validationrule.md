---
locale: en-US
source_revision: 4f55aceb4528bb2f23f6df7261d576123104df5097928d86a2b9915f6c2f9824
api_id: "validation::ValidationRule"
---
# validation::ValidationRule

Evaluate an exact decimal rule.

```rust
pub struct ValidationRule
```

## Parameters, return and normal usage

Create a rule with parse(tag, parameter), then reuse it for matches or validate. Rule parameters are checked once at construction; browser handles belong to one run and may be freed explicitly. Rules compare exact Decimal values without a floating-point conversion. decimal_required checks initialization, so initialized zero passes. eq/ne/gt/gte/lt/lte compare numerically even for uninitialized zero. between uses inclusive lower~upper and additionally requires initialization. positive/negative/nonzero require initialization. max_precision checks stored scale, including retained trailing zeros; it does not imply required.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule" locale="en-US" />

## Browser boundaries

The validation feature is enabled. Each string is capped at 8192 UTF-8 bytes; rule/error lists and message maps at 64 entries; JSON transport at 65536 bytes. Decimal rule parameters follow the [resource policy](../resource-policy). Translation preflights a conservative total substituted-text bound of 65536 bytes and may reject a just-fitting result. Custom tags and fields are data, never executed code. Validation failures, configuration errors and bridge resource errors remain distinct.

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
