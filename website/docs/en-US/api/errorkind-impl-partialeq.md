---
locale: en-US
source_revision: 5ee0e7b51aa068361991bd8a8b6016ae9f25f271ec57ec1d0990b39445f68b64
api_id: "ErrorKind::impl PartialEq"
---
# ErrorKind::impl PartialEq

Use the Rust PartialEq implementation.

```rust
impl PartialEq for ErrorKind
```

## Parameters, return and normal usage

Equality compares the complete error message/categories, or the exact enum variant. DecimalError equality is stricter than matching one category. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-partialeq" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
