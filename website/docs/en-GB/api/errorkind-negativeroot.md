---
locale: en-GB
source_revision: e2bc3815435c7e993973dc27c3f32515fd48346d012f56633ed09feff4f0aa99
translation_provenance: en-US-reviewed-reuse
api_id: "ErrorKind::NegativeRoot"
---
# ErrorKind::NegativeRoot

Identify NegativeRoot failures.

```rust
NegativeRoot
```

## Parameters, return and normal usage

An even root was requested for a negative value. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-negativeroot" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
