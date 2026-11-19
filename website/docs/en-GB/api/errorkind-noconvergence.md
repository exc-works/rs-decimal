---
locale: en-GB
source_revision: c50c700bf613d916c3c5e5c98c1711b754a7a2e86cbb6f64e10b64167cb79a2a
translation_provenance: en-US-reviewed-reuse
api_id: "ErrorKind::NoConvergence"
---
# ErrorKind::NoConvergence

Identify NoConvergence failures.

```rust
NoConvergence
```

## Parameters, return and normal usage

The bounded algorithm could not certify a rounded result; this does not mean the mathematical result does not exist.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-noconvergence" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
