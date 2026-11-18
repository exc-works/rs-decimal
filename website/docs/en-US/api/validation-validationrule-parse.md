---
locale: en-US
source_revision: e175d4d7de2a038dff837a099025c66d8337b4480eacf9bc8d2de1f1d5950025
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

<MethodExample id="validation-validation-validationrule-parse" locale="en-US" />

## Browser boundaries

The validation feature is enabled. Each string is capped at 8192 UTF-8 bytes; rule/error lists and message maps at 64 entries; JSON transport at 65536 bytes. Decimal rule parameters follow the [resource policy](../resource-policy). Translation preflights a conservative total substituted-text bound of 65536 bytes and may reject a just-fitting result. Custom tags and fields are data, never executed code. Validation failures, configuration errors and bridge resource errors remain distinct.

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
