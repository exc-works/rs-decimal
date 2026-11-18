---
locale: en-US
source_revision: e5103a64d713be93c5c51f691b143dc290327bd825e0acd2029d936b2165437e
api_id: "ScanValue::Float64"
---
# ScanValue::Float64

Select the Float64 SQL input variant.

```rust
Float64 (f64)
```

## Parameters, return and normal usage

Carry binary64. Finite values use the core shortest decimal conversion; NaN and infinities return Unmarshal. Already rounded JS numbers cannot recover original source digits. Variants are borrowed by scan and can be reused within one browser run; fabricated or foreign-run values are rejected.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float64" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
