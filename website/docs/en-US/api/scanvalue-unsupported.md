---
locale: en-US
source_revision: 174ec0afec71ca2dec4f0c9c844589a1e01782200d059f787f800837923cca1b
api_id: "ScanValue::Unsupported"
---
# ScanValue::Unsupported

Select the Unsupported SQL input variant.

```rust
Unsupported
```

## Parameters, return and normal usage

Report an unsupported driver input without coercion. scan returns Unmarshal and preserves the existing value. Variants are borrowed by scan and can be reused within one browser run; fabricated or foreign-run values are rejected.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-unsupported" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
