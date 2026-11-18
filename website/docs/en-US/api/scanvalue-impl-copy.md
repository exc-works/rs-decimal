---
locale: en-US
source_revision: 3a7f97312a58229a1540e8d6d6b0b0c1b31b50abeb603d9877b25c182c55eafa
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

<MethodExample id="scan-errors-scanvalue-impl-copy" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
