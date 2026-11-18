---
locale: en-US
source_revision: 1f1d7bd844459ed8638e77e456d476b7cad16313d7e3492984a6370cba407445
api_id: "ErrorKind::InvalidArgument"
---
# ErrorKind::InvalidArgument

Identify InvalidArgument failures.

```rust
InvalidArgument
```

## Parameters, return and normal usage

An argument, validation rule or message template is invalid. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidargument" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
