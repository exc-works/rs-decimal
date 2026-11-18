---
locale: en-US
source_revision: 65b82f1df9828e1e76dbeae405f27c44161cfdcb090640f8812099c14a0bd130
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

## Browser boundaries

The validation feature is enabled. Each string is capped at 8192 UTF-8 bytes; rule/error lists and message maps at 64 entries; JSON transport at 65536 bytes. Decimal rule parameters follow the [resource policy](../resource-policy). Translation preflights a conservative total substituted-text bound of 65536 bytes and may reject a just-fitting result. Custom tags and fields are data, never executed code. Validation failures, configuration errors and bridge resource errors remain distinct.

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
