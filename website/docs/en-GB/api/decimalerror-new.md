---
locale: en-GB
source_revision: 634fa480d38c38e977bcb656a0df8ea6f905528b83e0983585b20f7e954e4020
translation_provenance: en-US-reviewed-reuse
api_id: "DecimalError::new"
---
# DecimalError::new

Create one categorized error.

```rust
fn new (kind : ErrorKind , message : impl Into < String >) -> Self
```

## Parameters, return and normal usage

kind: ErrorKind, message: impl `Into<String>`/string; returns a new DecimalError. The message is human-readable and does not determine category membership. The browser named constructor creates a disposable Rust-backed value with message, matches(), kinds() and with_kind(). Thrown library exceptions are native JS Error objects with category="library" and a kinds array; use e.kinds.includes(ErrorKind.Unmarshal) there. Parameter/resource/moved bridge errors are separate and carry no library kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-new" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
