---
locale: en-GB
source_revision: 66f48b07b8a5c080eb29b01047825ba3448ab4e979a526ae96e347e482fef177
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="scan-errors-scanvalue-text" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
