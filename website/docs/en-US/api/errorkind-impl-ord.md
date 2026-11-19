---
locale: en-US
source_revision: 77c1d26d73dd77cab1d0c87534f9e5ec0b5d57b39f76087cd3ef2b288f8c6fae
api_id: "ErrorKind::impl Ord"
---
# ErrorKind::impl Ord

Use the Rust Ord implementation.

```rust
impl Ord for ErrorKind
```

## Parameters, return and normal usage

Orders ErrorKind by Rust declaration order, not severity or alphabetic name. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-ord" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
