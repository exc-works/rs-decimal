---
locale: en-US
source_revision: 788e3facaf7bb31d022529aaea08540d91c8a4001537b0085ab4e604ef0e3ae8
api_id: "validation::ValidationRule::matches"
---
# validation::ValidationRule::matches

Evaluate an exact decimal rule.

```rust
fn matches (& self , value : & Decimal) -> bool
```

## Parameters, return and normal usage

value: &Decimal/Decimal; returns bool/boolean and preserves both the rule and value. Rules compare exact Decimal values without a floating-point conversion. decimal_required checks initialization, so initialized zero passes. eq/ne/gt/gte/lt/lte compare numerically even for uninitialized zero. between uses inclusive lower~upper and additionally requires initialization. positive/negative/nonzero require initialization. max_precision checks stored scale, including retained trailing zeros; it does not imply required.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-matches" locale="en-US" />

## Browser boundaries

The validation feature is enabled. Each string is capped at 8192 UTF-8 bytes; rule/error lists and message maps at 64 entries; JSON transport at 65536 bytes. Decimal rule parameters follow the [resource policy](../resource-policy). Translation preflights a conservative total substituted-text bound of 65536 bytes and may reject a just-fitting result. Custom tags and fields are data, never executed code. Validation failures, configuration errors and bridge resource errors remain distinct.

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
