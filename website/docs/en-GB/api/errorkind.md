---
locale: en-GB
source_revision: 3bfbb5926961d9d892e3da2738bc0fc6133b23f056d8610803f8a02d24a533d4
translation_provenance: en-US-reviewed-reuse
api_id: "ErrorKind"
---
# ErrorKind

Describe stable library error categories.

```rust
pub enum ErrorKind
```

## Parameters, return and normal usage

The enum has eleven variants and can be copied and ordered in Rust. Browser ErrorKind is a frozen name map loaded from Rust. Use category membership, not a message comparison, to handle specific errors. The browser named constructor creates a disposable Rust-backed value with message, matches(), kinds() and with_kind(). Thrown library exceptions are native JS Error objects with category="library" and a kinds array; use e.kinds.includes(ErrorKind.Unmarshal) there. Parameter/resource/moved bridge errors are separate and carry no library kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
