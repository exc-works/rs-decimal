---
locale: en-GB
source_revision: 51d80d897f775a7466e06561586b9d500a34859b5d00dfee1f00a368a7a33f1e
translation_provenance: en-US-reviewed-reuse
api_id: "DecimalError::kinds"
---
# DecimalError::kinds

Read all error categories.

```rust
fn kinds (& self) -> & [ErrorKind]
```

## Parameters, return and normal usage

No parameters; returns &[ErrorKind]/an independent array of variant names. Categories are deduplicated and sorted in Rust enum order, not alphabetical order. The browser named constructor creates a disposable Rust-backed value with message, matches(), kinds() and with_kind(). Thrown library exceptions are native JS Error objects with category="library" and a kinds array; use e.kinds.includes(ErrorKind.Unmarshal) there. Parameter/resource/moved bridge errors are separate and carry no library kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-kinds" locale="en-GB" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
