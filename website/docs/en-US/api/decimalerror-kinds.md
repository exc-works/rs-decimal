---
locale: en-US
source_revision: 51be5745f7c3de3ef96d99823418ed54f498b3a9d7e9e49834c1ccd963cd3475
api_id: "DecimalError::kinds"
---
# DecimalError::kinds

Read all error categories.

```rust
fn kinds (& self) -> & [ErrorKind]
```

## Parameters, return and normal usage

No parameters; returns &[ErrorKind]/an independent array of variant names. Categories are deduplicated and sorted in Rust enum order, not alphabetical order. The browser named constructor creates a disposable Rust-backed value with message, matches(), kinds() and with_kind(). Thrown library exceptions are native JS Error objects with category="library" and a kinds array; use e.kinds.includes(ErrorKind.Unmarshal) there. Parameter/resource/moved bridge errors are separate and carry no library kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-kinds" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
