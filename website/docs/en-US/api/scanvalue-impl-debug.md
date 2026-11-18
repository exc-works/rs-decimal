---
locale: en-US
source_revision: 9436e2c2329251e2adecf17884000bc9408c12b9f0b7fd847a1b861820048eb7
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

<MethodExample id="scan-errors-scanvalue-impl-debug" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
