---
locale: en-GB
source_revision: b9b1d10da5aca99066127a0b0f1b72dc4589201f0d6b4df7f7823433f8faad44
translation_provenance: en-US-reviewed-reuse
api_id: "DecimalError::impl fmt :: Display::fmt"
---
# DecimalError::impl fmt :: Display::fmt

Use the Rust fmt  implementation.

```rust
fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result
```

## Parameters, return and normal usage

Display writes only the message. Use format! or to_string(); inspect kinds separately for machine-readable handling. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-fmt-display-fmt" locale="en-GB" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
