---
locale: en-GB
source_revision: f5ea64d7e7a6135acc2f55f3a40411c6d9222e20ac49981dab203cb401fa6651
translation_provenance: en-US-reviewed-reuse
api_id: reexport::one
---
# reexport::one

Import one from the rs_decimal crate root.

```rust
pub use decimal::one as one
```

## Normal usage and semantics

Use rs_decimal::one; this is the public import path for the same item, not an additional numerical implementation. JavaScript has no Rust use syntax; see the corresponding bridge reference.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-one" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
