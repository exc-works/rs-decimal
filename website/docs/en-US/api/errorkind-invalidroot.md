---
locale: en-US
source_revision: 04ccd2d93879c885eeb5c5c9d73e97e09115c8bc883b68449691de9e950117bb
api_id: "ErrorKind::InvalidRoot"
---
# ErrorKind::InvalidRoot

Identify InvalidRoot failures.

```rust
InvalidRoot
```

## Parameters, return and normal usage

Root degree is nonpositive. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidroot" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
