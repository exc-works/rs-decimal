---
locale: en-GB
source_revision: 725ef7f08917b0c306a1f269aaa8e188bc4212046b8491281df04a8f33b0e006
translation_provenance: en-US-reviewed-reuse
api_id: "DecimalError::impl fmt :: Display::fmt"
---
# DecimalError::impl fmt :: Display::fmt

Use the Rust fmt  implementation.

```rust
fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result
```

## Parameters, return and normal usage

Display writes only the message. Use format! or to_string(); inspect kinds separately for machine-readable handling. This is native Rust trait syntax; the browser does not expose it as a method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-fmt-display-fmt" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
