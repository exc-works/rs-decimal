---
locale: en-GB
source_revision: 8064cdb13ca114f6f1b29d727ed53f33f638f79926b35e606248ced643f822e9
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::value"
---
# Decimal::value

Get the canonical SQL value.

```rust
fn value (& self) -> String
```

## Parameters, return and normal usage

No parameters; returns String/string. Even uninitialized Decimal returns non-NULL "0"; use NullDecimal when SQL NULL is required. Removes fractional trailing zeros without changing scale or initialization. The returned text is a scalar value, not a quoted SQL query fragment.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-value" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
