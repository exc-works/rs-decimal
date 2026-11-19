---
locale: en-GB
source_revision: 15c94f02827b09a573140cb09fd7743d1624bd74d93875d8d0450c89214f2677
translation_provenance: en-US-reviewed-reuse
api_id: "ScanValue::Float32"
---
# ScanValue::Float32

Select the Float32 SQL input variant.

```rust
Float32 (f32)
```

## Parameters, return and normal usage

Carry binary32. The browser accepts number and Rust narrows it to f32 before scanning. NaN, infinities and narrowing overflow fail with Unmarshal. Variants are borrowed by scan and can be reused within one browser run; fabricated or foreign-run values are rejected.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float32" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
