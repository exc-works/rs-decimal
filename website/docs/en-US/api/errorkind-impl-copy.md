---
locale: en-US
source_revision: 473b8cc7f11457cef7cc926925c4f230754afa71c4dea38c85c3b954c8ca89fd
api_id: "ErrorKind::impl Copy"
---
# ErrorKind::impl Copy

Use the Rust Copy implementation.

```rust
impl Copy for ErrorKind
```

## Parameters, return and normal usage

Copy duplicates this small enum; borrowed scalar data is not deeply copied. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-copy" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
