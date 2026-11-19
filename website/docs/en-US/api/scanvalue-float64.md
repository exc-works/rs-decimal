---
locale: en-US
source_revision: b276bcddff5985f20e7905654e4b3cef6f352b3cfa139550a92b9425bd777e62
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

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
