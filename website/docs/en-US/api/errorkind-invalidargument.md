---
locale: en-US
source_revision: e14d22e2fbe70e02eda750145ed5b1a70252e171e63aeaafcfc01281be24e818
api_id: "ErrorKind::InvalidArgument"
---
# ErrorKind::InvalidArgument

Identify InvalidArgument failures.

```rust
InvalidArgument
```

## Parameters, return and normal usage

An argument, validation rule or message template is invalid. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidargument" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
