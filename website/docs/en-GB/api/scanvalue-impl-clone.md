---
locale: en-GB
source_revision: 11ee599a7357841e984cdaecadf4da635302996e14ae33191c323e57e59262ff
translation_provenance: en-US-reviewed-reuse
api_id: "ScanValue::impl Clone"
---
# ScanValue::impl Clone

Use the Rust Clone implementation.

```rust
impl Clone for ScanValue
```

## Parameters, return and normal usage

Clone preserves the complete value; borrowed ScanValue slices still refer to the original data. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-clone" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
