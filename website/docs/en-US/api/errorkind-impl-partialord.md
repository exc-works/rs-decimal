---
locale: en-US
source_revision: b6a1d6732343e8fec8b5e4584b880016ecb9ffdc8cf16c927c04637de618fc19
api_id: "ErrorKind::impl PartialOrd"
---
# ErrorKind::impl PartialOrd

Use the Rust PartialOrd implementation.

```rust
impl PartialOrd for ErrorKind
```

## Parameters, return and normal usage

Orders ErrorKind by Rust declaration order, not severity or alphabetic name. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-partialord" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
