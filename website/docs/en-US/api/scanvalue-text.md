---
locale: en-US
source_revision: b5a3d9a8d6370fd602bde442c0194ed8033b601f5d907d38c4cf9f51317197c6
api_id: "ScanValue::Text"
---
# ScanValue::Text

Select the Text SQL input variant.

```rust
Text (& 'a str)
```

## Parameters, return and normal usage

Borrow exact decimal text. A single matching pair of outer double quotes is stripped; escape sequences are not JSON-decoded. Nonzero trailing scale is preserved after parsing. Variants are borrowed by scan and can be reused within one browser run; fabricated or foreign-run values are rejected.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-text" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
