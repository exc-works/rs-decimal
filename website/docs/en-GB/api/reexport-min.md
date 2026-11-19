---
locale: en-GB
source_revision: f747bad3502ad5af9ea13bde6cf07e23bd90f835c390a356a17c381617af16c5
translation_provenance: en-US-reviewed-reuse
api_id: reexport::min
---
# reexport::min

Import min from the rs_decimal crate root.

```rust
pub use decimal::min as min
```

## Normal usage and semantics

Use rs_decimal::min; this is the public import path for the same item, not an additional numerical implementation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-min" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
