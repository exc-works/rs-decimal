---
locale: en-GB
source_revision: a985525078f122e92ef2d70bf9a962a63b82b868ba67925f976407356bf4ff6b
translation_provenance: en-US-reviewed-reuse
api_id: "DecimalError::kinds"
---
# DecimalError::kinds

Read all error categories.

```rust
fn kinds (& self) -> & [ErrorKind]
```

## Parameters, return and normal usage

No parameters; returns &[ErrorKind]/an independent array of variant names. Categories are deduplicated and sorted in Rust enum order, not alphabetical order. The browser named constructor creates a disposable Rust-backed value with message, matches(), kinds() and with_kind(). Thrown library exceptions are native JS Error objects with category="library" and a kinds array; use e.kinds.includes(ErrorKind.Unmarshal) there. Parameter/resource/moved errors are separate and carry no library kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-kinds" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
