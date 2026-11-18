---
locale: en-US
source_revision: 2c125881cbf08b883eeea1f8f7ddb1472b5bb7418b552e6c5a9a55f4ae4adeb9
api_id: "ErrorKind::NegativeRoot"
---
# ErrorKind::NegativeRoot

Identify NegativeRoot failures.

```rust
NegativeRoot
```

## Parameters, return and normal usage

An even root was requested for a negative value. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-negativeroot" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
