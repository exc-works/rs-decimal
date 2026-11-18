---
locale: en-GB
source_revision: 93c9a590df79625539c288c9f6a42d27aa1dba571b28363b98f9ec4642104e16
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="scan-errors-errorkind-impl-ord" locale="en-GB" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
