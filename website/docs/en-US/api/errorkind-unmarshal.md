---
locale: en-US
source_revision: c870cbfccba48189ea2c1f73c60ffc0835f379cdb13fc7e1adf9905fa90662b8
api_id: "ErrorKind::Unmarshal"
---
# ErrorKind::Unmarshal

Identify Unmarshal failures.

```rust
Unmarshal
```

## Parameters, return and normal usage

Decoding failed. Another category such as InvalidFormat may describe its cause; test membership rather than assuming one kind. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-unmarshal" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
