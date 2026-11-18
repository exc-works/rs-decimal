---
locale: en-GB
source_revision: 5aa2d7144a94e759cd81dc307aff2c18ce7db36a83dc3e91c6d08804e1cdf2c5
translation_provenance: en-US-reviewed-reuse
api_id: "DecimalError::impl PartialEq"
---
# DecimalError::impl PartialEq

Use the Rust PartialEq implementation.

```rust
impl PartialEq for DecimalError
```

## Parameters, return and normal usage

Equality compares the complete error message/categories, or the exact enum variant. DecimalError equality is stricter than matching one category. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-partialeq" locale="en-GB" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
