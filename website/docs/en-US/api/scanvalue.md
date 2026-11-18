---
locale: en-US
source_revision: bc03546de8ea5c2e53e57f5ce62fd19b75d7cd5d6737f4aa1452dbcef13f206d
api_id: "ScanValue"
---
# ScanValue

Select a driver-neutral typed SQL input.

```rust
pub enum ScanValue< 'a >
```

## Parameters, return and normal usage

The enum distinguishes NULL, f32, f64, i64, text, byte text and unsupported values. Choose an explicit variant before calling Decimal.scan or NullDecimal.scan. Borrowed text/bytes need only remain valid for the call in Rust; browser Bytes retains the caller array and observes edits made before scan. No database service is contacted.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
