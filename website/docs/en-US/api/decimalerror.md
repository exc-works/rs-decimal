---
locale: en-US
source_revision: 51df12e136a0b4ed98ea9f557c4e61c7872827fb92cbe6a97475d66177b43284
api_id: "DecimalError"
---
# DecimalError

Retain a message and one or more library error kinds.

```rust
pub struct DecimalError
```

## Parameters, return and normal usage

DecimalError stores a human-readable message and ordered, deduplicated categories. with_kind consumes the value; matches and kinds borrow it. It implements Rust Error and Display. The browser named constructor creates a disposable Rust-backed value with message, matches(), kinds() and with_kind(). Thrown library exceptions are native JS Error objects with category="library" and a kinds array; use e.kinds.includes(ErrorKind.Unmarshal) there. Parameter/resource/moved errors are separate and carry no library kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
