---
locale: en-GB
source_revision: 65e698cc918f9afeb389a84627e95883beff1af27e6fb5183194df8bba07cbc9
translation_provenance: en-US-reviewed-reuse
api_id: "ErrorKind::DivideByZero"
---
# ErrorKind::DivideByZero

Identify DivideByZero failures.

```rust
DivideByZero
```

## Parameters, return and normal usage

A divisor is zero. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-dividebyzero" locale="en-GB" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
