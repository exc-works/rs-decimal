---
locale: en-GB
source_revision: 906bb0bd7555c68ed2da4c7be205d647a1ca94b724e2e70f5d1160d968357278
translation_provenance: en-US-reviewed-reuse
api_id: "validation::ValidationRule::parse"
---
# validation::ValidationRule::parse

Evaluate an exact decimal rule.

```rust
fn parse (tag : & str , parameter : & str) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

tag: &str/string, parameter: &str/string; returns `Result<ValidationRule, DecimalError>`/a rule handle. Use full tags such as decimal_gt. Comparison parameters are decimal text; between takes lower~upper; max_precision takes a nonnegative isize. Unknown tags, malformed parameters and reversed ranges return InvalidArgument. On wasm32, isize is signed 32-bit. Parameter-free rules retain but do not interpret parameter text. Rules compare exact Decimal values without a floating-point conversion. decimal_required checks initialization, so initialized zero passes. eq/ne/gt/gte/lt/lte compare numerically even for uninitialized zero. between uses inclusive lower~upper and additionally requires initialization. positive/negative/nonzero require initialization. max_precision checks stored scale, including retained trailing zeros; it does not imply required.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-parse" locale="en-GB" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
