---
locale: en-GB
source_revision: 78277d1082dac8a98f79b7018413113c09afb0c224a6c82ae6c179432b0517f1
translation_provenance: en-US-reviewed-reuse
api_id: "ScanValue::Unsupported"
---
# ScanValue::Unsupported

Select the Unsupported SQL input variant.

```rust
Unsupported
```

## Parameters, return and normal usage

Report an unsupported driver input without coercion. scan returns Unmarshal and preserves the existing value. Variants are borrowed by scan and can be reused within one browser run; fabricated or foreign-run values are rejected.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-unsupported" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
