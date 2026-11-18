---
locale: en-US
source_revision: 8ab3953570c1516f55bc48b7c251f1cfbcd5ae142c6f5206aaaaa5635e651b35
api_id: "DecimalError::impl Debug"
---
# DecimalError::impl Debug

Use the Rust Debug implementation.

```rust
impl Debug for DecimalError
```

## Parameters, return and normal usage

Debug provides diagnostic text, not a stable wire serialization. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-debug" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
