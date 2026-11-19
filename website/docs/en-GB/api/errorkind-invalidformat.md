---
locale: en-GB
source_revision: 72523519c8ccf1aad4c8baebb6ac1ef42e8070b9cc7f7f72b600ff77781eaf4e
translation_provenance: en-US-reviewed-reuse
api_id: "ErrorKind::InvalidFormat"
---
# ErrorKind::InvalidFormat

Identify InvalidFormat failures.

```rust
InvalidFormat
```

## Parameters, return and normal usage

Input is not a supported finite decimal representation. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidformat" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
