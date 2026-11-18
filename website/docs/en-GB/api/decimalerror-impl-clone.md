---
locale: en-GB
source_revision: 401f5b53ed7c2747c248cad36ae59554a16f09a1f304c6c8d0b65cf689946b2f
translation_provenance: en-US-reviewed-reuse
api_id: "DecimalError::impl Clone"
---
# DecimalError::impl Clone

Use the Rust Clone implementation.

```rust
impl Clone for DecimalError
```

## Parameters, return and normal usage

Clone preserves the complete value; borrowed ScanValue slices still refer to the original data. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-clone" locale="en-GB" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
