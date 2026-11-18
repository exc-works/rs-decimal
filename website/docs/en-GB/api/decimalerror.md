---
locale: en-GB
source_revision: e7411932b8884630967e9c3e34bf9db436dc8920df8139ae778ac83fc80fdcb3
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="scan-errors-decimalerror" locale="en-GB" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
