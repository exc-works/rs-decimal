---
locale: en-US
source_revision: 76aa255f8bc8b60ca22e7c6936a048607e56eb1eaa641962626233731b182883
api_id: "reexport::ErrorKind"
---
# reexport::ErrorKind

Import ErrorKind from the crate root.

```rust
pub use error::ErrorKind as ErrorKind
```

## Parameters, return and normal usage

Use rs_decimal::ErrorKind for the same public item. This is an import path, not an additional implementation or browser method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-reexport-errorkind" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
