---
locale: en-GB
source_revision: 292844a00b6e84cbd25e16d6e84fa03c5565c434cf287f28e2959e5ca0c46780
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="scan-errors-decimalerror-matches" locale="en-GB" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
