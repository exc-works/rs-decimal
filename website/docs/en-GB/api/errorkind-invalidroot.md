---
locale: en-GB
source_revision: 5acf1a9800c0d02bd1cb1fa68c2c1f7068c1bb36cf20034e0a1e404758ce2bc4
translation_provenance: en-US-reviewed-reuse
api_id: "ErrorKind::InvalidRoot"
---
# ErrorKind::InvalidRoot

Identify InvalidRoot failures.

```rust
InvalidRoot
```

## Parameters, return and normal usage

Root degree is nonpositive. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidroot" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
