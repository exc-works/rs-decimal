---
locale: en-US
source_revision: 52cfb6e6f9d640e445d0e6b774455d4a9cdc44bf1d1bc94d3efd717749548fc4
api_id: "DecimalError::new"
---
# DecimalError::new

Create one categorized error.

```rust
fn new (kind : ErrorKind , message : impl Into < String >) -> Self
```

## Parameters, return and normal usage

kind: ErrorKind, message: impl `Into<String>`/string; returns a new DecimalError. The message is human-readable and does not determine category membership. The browser named constructor creates a disposable Rust-backed value with message, matches(), kinds() and with_kind(). Thrown library exceptions are native JS Error objects with category="library" and a kinds array; use e.kinds.includes(ErrorKind.Unmarshal) there. Parameter/resource/moved errors are separate and carry no library kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-new" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
