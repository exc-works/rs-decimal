---
locale: en-US
source_revision: 8a30b5196c371952fcc2508159879f3619eaf7085b839003d31c6721b7f9cf86
api_id: "validation"
---
# validation

Collect ordered rule failures and translate them.

```rust
pub mod validation
```

## Parameters, return and normal usage

Rust requires the validation feature, enabled in the browser build. The module groups exact rule evaluation and owned message templates without a framework or global registry. In the browser, use the validation namespace. Field names are caller text; translation substitutes them literally. Use required explicitly when missing input must fail.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation" locale="en-US" />

## Browser boundaries

The validation feature is enabled. Each string is capped at 8192 UTF-8 bytes; rule/error lists and message maps at 64 entries; JSON transport at 65536 bytes. Decimal rule parameters follow the [resource policy](../resource-policy). Translation preflights a conservative total substituted-text bound of 65536 bytes and may reject a just-fitting result. Custom tags and fields are data, never executed code. Validation failures, configuration errors and bridge resource errors remain distinct.

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
