---
locale: en-US
source_revision: 2bcf37ee1521720789e7103cecdce4020a906f8f85dfb286e500fd7c881d8638
api_id: "ErrorKind::InvalidFormat"
---
# ErrorKind::InvalidFormat

Identify InvalidFormat failures.

```rust
InvalidFormat
```

## Parameters, return and normal usage

Input is not a supported finite decimal representation. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidformat" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
