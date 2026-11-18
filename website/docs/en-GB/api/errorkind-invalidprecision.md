---
locale: en-GB
source_revision: f8d4a19c8dfdd338ced205f1c85d4c5643f5573e19f1a55cd3492da89573f098
translation_provenance: en-US-reviewed-reuse
api_id: "ErrorKind::InvalidPrecision"
---
# ErrorKind::InvalidPrecision

Identify InvalidPrecision failures.

```rust
InvalidPrecision
```

## Parameters, return and normal usage

Requested precision is invalid. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidprecision" locale="en-GB" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
