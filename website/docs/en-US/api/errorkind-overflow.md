---
locale: en-US
source_revision: b5e19b2f97fd7e9d8a942453bc1d3336bd82a5f0e33e943baa1dc21eb3e5ac5f
api_id: "ErrorKind::Overflow"
---
# ErrorKind::Overflow

Identify Overflow failures.

```rust
Overflow
```

## Parameters, return and normal usage

A coefficient, scale or operation exceeds the core supported range. Browser resource errors are separate. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-overflow" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
