---
locale: en-US
source_revision: e6b8dfbe3d3140a4697ec5a94f4d508f975bc9c0847e91c70ef29e95978ddcfb
api_id: "ErrorKind"
---
# ErrorKind

Describe stable library error categories.

```rust
pub enum ErrorKind
```

## Parameters, return and normal usage

The enum has eleven variants and can be copied and ordered in Rust. Browser ErrorKind is a frozen name map loaded from Rust. Use category membership, not a message comparison, to handle specific errors. The browser named constructor creates a disposable Rust-backed value with message, matches(), kinds() and with_kind(). Thrown library exceptions are native JS Error objects with category="library" and a kinds array; use e.kinds.includes(ErrorKind.Unmarshal) there. Parameter/resource/moved errors are separate and carry no library kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
