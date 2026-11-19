---
locale: en-GB
source_revision: 7eeabc8ee1ace4f862b185d7c83f7d0a53594b9ff67ac15d988dde4ffaa12e4b
translation_provenance: en-US-reviewed-reuse
api_id: "ScanValue::impl Copy"
---
# ScanValue::impl Copy

Use the Rust Copy implementation.

```rust
impl Copy for ScanValue
```

## Parameters, return and normal usage

Copy duplicates this small enum; borrowed scalar data is not deeply copied. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-copy" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
