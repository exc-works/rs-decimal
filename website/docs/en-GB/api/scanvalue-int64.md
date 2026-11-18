---
locale: en-GB
source_revision: 36f5ff0720c14718444a86498f3f5cd1d16ba0df1d4c60c2d59357d8e469675e
translation_provenance: en-US-reviewed-reuse
api_id: "ScanValue::Int64"
---
# ScanValue::Int64

Select the Int64 SQL input variant.

```rust
Int64 (i64)
```

## Parameters, return and normal usage

Carry an i64 exactly. The browser requires bigint in the signed 64-bit range; passing a JS number is rejected. Variants are borrowed by scan and can be reused within one browser run; fabricated or foreign-run values are rejected.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-int64" locale="en-GB" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
