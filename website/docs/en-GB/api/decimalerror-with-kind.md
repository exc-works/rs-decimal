---
locale: en-GB
source_revision: f5327cbdbb40212ac4046096b90742ab088011d23930faef1e12c326d8e03e09
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="scan-errors-decimalerror-with-kind" locale="en-GB" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
