---
locale: en-US
source_revision: 5b92e8602e178e484bc5b465cd3d2014888bc1d25a5a0633a6502326ad20ca84
api_id: "ErrorKind::RoundUnnecessary"
---
# ErrorKind::RoundUnnecessary

Identify RoundUnnecessary failures.

```rust
RoundUnnecessary
```

## Parameters, return and normal usage

Exact rounding was requested but nonzero digits would be discarded. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-roundunnecessary" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
