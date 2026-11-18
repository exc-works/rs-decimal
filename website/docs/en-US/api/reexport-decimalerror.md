---
locale: en-US
source_revision: 0dd0f243213461bb54e73305b48db01cf1b491f68e6951552c4179ef98edf6bf
api_id: "reexport::DecimalError"
---
# reexport::DecimalError

Import DecimalError from the crate root.

```rust
pub use error::DecimalError as DecimalError
```

## Parameters, return and normal usage

Use rs_decimal::DecimalError for the same public item. This is an import path, not an additional implementation or browser method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-reexport-decimalerror" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
