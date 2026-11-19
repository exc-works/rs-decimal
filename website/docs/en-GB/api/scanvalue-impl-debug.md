---
locale: en-GB
source_revision: ff9a08f2b3feca78c7b4250dd2c69ee67ba05009b21ca77af28e0824e74ae8d9
translation_provenance: en-US-reviewed-reuse
api_id: "ScanValue::impl Debug"
---
# ScanValue::impl Debug

Use the Rust Debug implementation.

```rust
impl Debug for ScanValue
```

## Parameters, return and normal usage

Debug provides diagnostic text, not a stable wire serialization. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-debug" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
