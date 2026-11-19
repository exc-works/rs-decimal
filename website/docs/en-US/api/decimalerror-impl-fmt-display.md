---
locale: en-US
source_revision: b479610db2fc4071c7650b3bc2c08969f87f046ec9147a92c1fba0a74d41a23f
api_id: "DecimalError::impl fmt :: Display"
---
# DecimalError::impl fmt :: Display

Use the Rust fmt  implementation.

```rust
impl fmt :: Display for DecimalError 
```

## Parameters, return and normal usage

Display writes only the message. Use format! or to_string(); inspect kinds separately for machine-readable handling. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-fmt-display" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
