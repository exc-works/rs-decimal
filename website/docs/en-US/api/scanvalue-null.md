---
locale: en-US
source_revision: 17f954a2ac80c90ce4a20dd8e36b0981b73e6748171ba829abe55cdfdc69b2b3
api_id: "ScanValue::Null"
---
# ScanValue::Null

Select the Null SQL input variant.

```rust
Null
```

## Parameters, return and normal usage

Represent SQL NULL; scanning resets Decimal to uninitialized zero and clears NullDecimal. Variants are borrowed by scan and can be reused within one browser run; fabricated or foreign-run values are rejected.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-null" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
