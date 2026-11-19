---
locale: en-US
source_revision: 6eb93b97fb5de84f7620ea6f0aa7d8d8ce7bbe38e3bab65b24ca028dc1ed97cd
api_id: "ErrorKind::Overflow"
---
# ErrorKind::Overflow

Identify Overflow failures.

```rust
Overflow
```

## Parameters, return and normal usage

A coefficient, scale or operation exceeds the core supported range. Browser resource errors are separate. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-overflow" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
