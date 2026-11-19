---
locale: en-US
source_revision: ecc694dc9095e4bf91fee90f15b14f69b82899c7811b1a6abfdda76dd5e06c56
api_id: "DecimalError::with_kind"
---
# DecimalError::with_kind

Consume an error and add a category.

```rust
fn with_kind (mut self , kind : ErrorKind) -> Self
```

## Parameters, return and normal usage

kind: ErrorKind; consumes self and returns a new DecimalError. Existing categories and message are preserved; duplicates are ignored. The original browser handle becomes unusable. Invalid JS kind values are rejected before the move. The browser named constructor creates a disposable Rust-backed value with message, matches(), kinds() and with_kind(). Thrown library exceptions are native JS Error objects with category="library" and a kinds array; use e.kinds.includes(ErrorKind.Unmarshal) there. Parameter/resource/moved bridge errors are separate and carry no library kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-with-kind" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
