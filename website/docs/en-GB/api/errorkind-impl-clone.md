---
locale: en-GB
source_revision: b4b7e965750a7655fdeb57385feedb3c62874c4ab66fdfb6a3f2062d579960d6
translation_provenance: en-US-reviewed-reuse
api_id: "ErrorKind::impl Clone"
---
# ErrorKind::impl Clone

Use the Rust Clone implementation.

```rust
impl Clone for ErrorKind
```

## Parameters, return and normal usage

Clone preserves the complete value; borrowed ScanValue slices still refer to the original data. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-clone" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
