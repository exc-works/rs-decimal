---
locale: en-GB
source_revision: 29e91f626fd685fb0b5987fc104c1ba0e2752897194773305e3d64dcc1b16f1a
translation_provenance: en-US-reviewed-reuse
api_id: reexport::one
---
# reexport::one

Import one from the rs_decimal crate root.

```rust
pub use decimal::one as one
```

## Normal usage and semantics

Use rs_decimal::one; this is the public import path for the same item, not an additional numerical implementation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-one" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
