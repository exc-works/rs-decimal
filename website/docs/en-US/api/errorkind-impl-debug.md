---
locale: en-US
source_revision: 1b410a44c40ddf1ed7ab08ecafd94bf955a8c6e8ace55716beba7530f0db9122
api_id: "ErrorKind::impl Debug"
---
# ErrorKind::impl Debug

Use the Rust Debug implementation.

```rust
impl Debug for ErrorKind
```

## Parameters, return and normal usage

Debug provides diagnostic text, not a stable wire serialization. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-debug" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
