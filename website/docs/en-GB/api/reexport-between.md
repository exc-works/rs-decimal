---
locale: en-GB
source_revision: 3300205234593fd9e0974d13d203dba364588cd233c7dd62f42452687449a8f7
translation_provenance: en-US-reviewed-reuse
api_id: reexport::between
---
# reexport::between

Import between from the rs_decimal crate root.

```rust
pub use decimal::between as between
```

## Normal usage and semantics

Use rs_decimal::between; this is the public import path for the same item, not an additional numerical implementation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-between" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
