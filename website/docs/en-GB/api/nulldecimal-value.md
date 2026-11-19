---
locale: en-GB
source_revision: cd65ee6368cc890889ec3fceb39f3b12814d9f2d0fe811e6c57c730bc5d81b5b
translation_provenance: en-US-reviewed-reuse
api_id: "NullDecimal::value"
---
# NullDecimal::value

Get the canonical SQL value.

```rust
fn value (& self) -> Option < String >
```

## Parameters, return and normal usage

No parameters; returns `Option<String>`/string | null. Invalid nullable values return None/null, regardless of hidden inner data. Valid values return canonical decimal text, including "0" for an uninitialized inner zero. Removes fractional trailing zeros without changing scale or initialization. The returned text is a scalar value, not a quoted SQL query fragment.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-value" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
