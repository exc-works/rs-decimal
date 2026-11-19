---
locale: en-US
source_revision: 9dbbcca20e9ec70948f275a66820b5aebd2bfa7f33ae4f382b6d22c025c682c3
api_id: "DecimalError::impl Clone"
---
# DecimalError::impl Clone

Use the Rust Clone implementation.

```rust
impl Clone for DecimalError
```

## Parameters, return and normal usage

Clone preserves the complete value; borrowed ScanValue slices still refer to the original data. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-clone" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
