---
locale: en-GB
source_revision: d3605c64b7e680ec58960c25358a321ed68015b81953ff878939d179147793d3
translation_provenance: en-US-reviewed-reuse
api_id: "DecimalError::impl Error"
---
# DecimalError::impl Error

Use the Rust Error implementation.

```rust
impl Error for DecimalError
```

## Parameters, return and normal usage

Use as a standard Rust Error, including boxed error propagation. Additional categories are not a chained source; source() is None. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-error" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
