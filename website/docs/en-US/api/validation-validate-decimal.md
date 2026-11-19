---
locale: en-US
source_revision: 53a002a881140d83f1a447d3f2f73f066b75a4b8a71f55b044742e2243254251
api_id: "validation::validate_decimal"
---
# validation::validate_decimal

Collect ordered rule failures and translate them.

```rust
fn validate_decimal (field : & str , value : & Decimal , rules : & [ValidationRule] ,) -> Vec < ValidationError >
```

## Parameters, return and normal usage

Rust requires the validation feature, enabled in the browser build. field: &str/string, value: &Decimal/Decimal, rules: &[ValidationRule]/array; returns every failing ValidationError in caller order. Duplicate rules may produce duplicate failures; no reflection, registry or early exit is used. Empty rules return an empty array. All inputs are borrowed. Field names are caller text; translation substitutes them literally. Use required explicitly when missing input must fail.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validate-decimal" locale="en-US" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
