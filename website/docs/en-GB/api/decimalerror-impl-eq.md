---
locale: en-GB
source_revision: cbf84996c156999f0cfb9cac49a1135ecf0d19ca8aba3abbedbed0acff443c0a
translation_provenance: en-US-reviewed-reuse
api_id: "DecimalError::impl Eq"
---
# DecimalError::impl Eq

Use the Rust Eq implementation.

```rust
impl Eq for DecimalError
```

## Parameters, return and normal usage

Equality compares the complete error message/categories, or the exact enum variant. DecimalError equality is stricter than matching one category. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-eq" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
