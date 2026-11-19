---
locale: en-US
source_revision: 8805baa34ac15d1bc6208bedc88ae9d1d05c8235150f9f2331a9105561b652f3
api_id: "DecimalError"
---
# DecimalError

Retain a message and one or more library error kinds.

```rust
pub struct DecimalError
```

## Parameters, return and normal usage

DecimalError stores a human-readable message and ordered, deduplicated categories. with_kind consumes the value; matches and kinds borrow it. It implements Rust Error and Display. The browser named constructor creates a disposable Rust-backed value with message, matches(), kinds() and with_kind(). Thrown library exceptions are native JS Error objects with category="library" and a kinds array; use e.kinds.includes(ErrorKind.Unmarshal) there. Parameter/resource/moved bridge errors are separate and carry no library kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
