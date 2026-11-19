---
locale: en-US
source_revision: 69334c1aa4ddf9b63780cdd0f7b3c783c6e1646067bf4562ab026efd47279431
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

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
