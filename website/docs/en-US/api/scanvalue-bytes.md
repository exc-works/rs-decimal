---
locale: en-US
source_revision: 4a909248a2e14bd655a50dd72e826f6d87a3347b735e76ddfe31ae9a4d4b4671
api_id: "ScanValue::Bytes"
---
# ScanValue::Bytes

Select the Bytes SQL input variant.

```rust
Bytes (& 'a [u8])
```

## Parameters, return and normal usage

Borrow decimal-text bytes, not binary/BSON wire data. The browser retains the Uint8Array until scan; edits before scan are observed. scan never mutates the input. Invalid UTF-8 fails decimal parsing. Variants are borrowed by scan and can be reused within one browser run; fabricated or foreign-run values are rejected.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-bytes" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
