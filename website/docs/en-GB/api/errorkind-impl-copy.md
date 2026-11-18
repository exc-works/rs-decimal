---
locale: en-GB
source_revision: a66891e5a69e79ad1f199a0dd34da946899f3deff1662608f76ae3c569145632
translation_provenance: en-US-reviewed-reuse
api_id: "ErrorKind::impl Copy"
---
# ErrorKind::impl Copy

Use the Rust Copy implementation.

```rust
impl Copy for ErrorKind
```

## Parameters, return and normal usage

Copy duplicates this small enum; borrowed scalar data is not deeply copied. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-copy" locale="en-GB" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
