---
locale: en-US
source_revision: bceeb7ede6fc6a0450eec9bb9848cbe43ca4c0d990bf02edf9131f531001c34f
api_id: "DecimalError::impl PartialEq"
---
# DecimalError::impl PartialEq

Use the Rust PartialEq implementation.

```rust
impl PartialEq for DecimalError
```

## Parameters, return and normal usage

Equality compares the complete error message/categories, or the exact enum variant. DecimalError equality is stricter than matching one category. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-partialeq" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
