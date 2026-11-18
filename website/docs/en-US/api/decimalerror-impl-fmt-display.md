---
locale: en-US
source_revision: 313aa913e617945bd29e0e39f2ee0488ae3c0d32ae43ffdacbbf7badbfb3dbb1
api_id: "DecimalError::impl fmt :: Display"
---
# DecimalError::impl fmt :: Display

Use the Rust fmt  implementation.

```rust
impl fmt :: Display for DecimalError 
```

## Parameters, return and normal usage

Display writes only the message. Use format! or to_string(); inspect kinds separately for machine-readable handling. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-fmt-display" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
