---
locale: en-US
source_revision: 1534eba12b3e65df06a4bea76e8dd9c9cd12eeb6bcfbce92f69f83b923eef4c3
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

<MethodExample id="scan-errors-scanvalue-int64" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
