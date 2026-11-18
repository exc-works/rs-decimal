---
locale: en-US
source_revision: ab8ea6bf03391270a33235fc649739ef25b1cdab822e201005d40879c5be9299
api_id: "Decimal::value"
---
# Decimal::value

Get the canonical SQL value.

```rust
fn value (& self) -> String
```

## Parameters, return and normal usage

No parameters; returns String/string. Even uninitialized Decimal returns non-NULL "0"; use NullDecimal when SQL NULL is required. Removes fractional trailing zeros without changing scale or initialization. The returned text is a scalar value, not a quoted SQL query fragment.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-value" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
