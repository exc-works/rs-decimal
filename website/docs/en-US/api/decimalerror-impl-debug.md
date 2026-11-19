---
locale: en-US
source_revision: f91df84f154c9cc1b9dd7b8a5909be0f1702eb57d28ff7a024367e2c25d73638
api_id: "DecimalError::impl Debug"
---
# DecimalError::impl Debug

Use the Rust Debug implementation.

```rust
impl Debug for DecimalError
```

## Parameters, return and normal usage

Debug provides diagnostic text, not a stable wire serialization. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-debug" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
