---
locale: en-GB
source_revision: 79298c474720d5ca067e6e36ecdefe721611687b58bab5fff1d35e1889208bcd
translation_provenance: en-US-reviewed-reuse
api_id: "NullDecimal::value"
---
# NullDecimal::value

Get the canonical SQL value.

```rust
fn value (& self) -> Option < String >
```

## Parameters, return and normal usage

No parameters; returns `Option<String>`/string | null. Invalid nullable values return None/null, regardless of hidden inner data. Valid values return canonical decimal text, including "0" for an uninitialized inner zero. Removes fractional trailing zeros without changing scale or initialization. The returned text is a scalar value, not a quoted SQL query fragment.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-value" locale="en-GB" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
