---
locale: en-US
source_revision: c136bea39fff64039ef208c269a0655f2c126075df944740bc4d24f9d953ef55
api_id: "ErrorKind"
---
# ErrorKind

Describe stable library error categories.

```rust
pub enum ErrorKind
```

## Parameters, return and normal usage

The enum has eleven variants and can be copied and ordered in Rust. Browser ErrorKind is a frozen name map loaded from Rust. Use category membership, not a message comparison, to handle specific errors. The browser named constructor creates a disposable Rust-backed value with message, matches(), kinds() and with_kind(). Thrown library exceptions are native JS Error objects with category="library" and a kinds array; use e.kinds.includes(ErrorKind.Unmarshal) there. Parameter/resource/moved bridge errors are separate and carry no library kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
