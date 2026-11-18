---
locale: en-US
source_revision: ad02136caf1af8e3aa280c1a4cea67c286a29bad87537bf02a0815db1559db64
api_id: "ErrorKind::InvalidLog"
---
# ErrorKind::InvalidLog

Identify InvalidLog failures.

```rust
InvalidLog
```

## Parameters, return and normal usage

A logarithm input is nonpositive. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidlog" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
