---
locale: en-GB
source_revision: c5bc8b97bab18c557c0918fb7ad15cabc2fd824c618c469b8f007d1f6299272b
translation_provenance: en-US-reviewed-reuse
api_id: "ErrorKind::DivideByZero"
---
# ErrorKind::DivideByZero

Identify DivideByZero failures.

```rust
DivideByZero
```

## Parameters, return and normal usage

A divisor is zero. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-dividebyzero" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
