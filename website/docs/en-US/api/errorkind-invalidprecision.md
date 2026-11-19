---
locale: en-US
source_revision: 68b84662c55e7239d15b1fe54a5a2076eb45cea41916719d90f7dbe8ab9e7150
api_id: "ErrorKind::InvalidPrecision"
---
# ErrorKind::InvalidPrecision

Identify InvalidPrecision failures.

```rust
InvalidPrecision
```

## Parameters, return and normal usage

Requested precision is invalid. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidprecision" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
