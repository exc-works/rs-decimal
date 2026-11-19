---
locale: en-GB
source_revision: 816a5e8449654efb901339ad5516b3fdb94be2af25e3c709a61823154b83be91
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="validation-validation-validationrule-matches" locale="en-GB" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
