---
locale: en-US
source_revision: 8106ef71942ee657518a42cba33a247697b1ac2ebfcf011160e32c4adeb6e0b5
api_id: "reexport::ScanValue"
---
# reexport::ScanValue

Import ScanValue from the crate root.

```rust
pub use sql::ScanValue as ScanValue
```

## Parameters, return and normal usage

Use rs_decimal::ScanValue for the same public item. This is an import path, not an additional implementation or browser method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-reexport-scanvalue" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
