---
locale: en-US
source_revision: 4a04c1296f4a9e4d0c1eac8d1707242f3d7b87d7302eac99dc6a590f2c03208e
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

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
