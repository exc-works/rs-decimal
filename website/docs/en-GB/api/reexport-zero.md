---
locale: en-GB
source_revision: d9e6c46654b05ae244bd56f7035b17e55d6f30ce2bee3665fad9be992491dd6d
translation_provenance: en-US-reviewed-reuse
api_id: reexport::zero
---
# reexport::zero

Import zero from the rs_decimal crate root.

```rust
pub use decimal::zero as zero
```

## Normal usage and semantics

Use rs_decimal::zero; this is the public import path for the same item, not an additional numerical implementation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-zero" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
