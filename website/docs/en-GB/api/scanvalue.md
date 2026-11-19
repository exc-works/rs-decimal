---
locale: en-GB
source_revision: 1d04e1207bda167aea5c52bc9b525b3aa5208f7c0d21a8e9e4cdf61685f69f69
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="scan-errors-scanvalue" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
