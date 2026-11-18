---
locale: en-US
source_revision: bdf3f59a65c991206a82fedc78f9d1dd8953abdf9e7ea6aea954a760e99e1ac6
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

<MethodExample id="scan-errors-scanvalue-impl-clone" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
