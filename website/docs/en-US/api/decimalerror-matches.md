---
locale: en-US
source_revision: 51d8eb06be2cf1b2c76ae713dc34443717bd6adc00c52dec95e805914eed6ec1
api_id: "DecimalError::matches"
---
# DecimalError::matches

Test category membership.

```rust
fn matches (& self , kind : ErrorKind) -> bool
```

## Parameters, return and normal usage

kind: ErrorKind; returns bool/boolean. Borrowing does not modify the error. An error can match multiple kinds at once. The browser named constructor creates a disposable Rust-backed value with message, matches(), kinds() and with_kind(). Thrown library exceptions are native JS Error objects with category="library" and a kinds array; use e.kinds.includes(ErrorKind.Unmarshal) there. Parameter/resource/moved bridge errors are separate and carry no library kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-matches" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
