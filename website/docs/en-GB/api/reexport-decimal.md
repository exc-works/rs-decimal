---
locale: en-GB
source_revision: 6976205af89f0a7bb675a6147933c2fe624e8c9d96703f7f354c0cb5ca52f755
translation_provenance: en-US-reviewed-reuse
api_id: reexport::Decimal
---
# reexport::Decimal

Import Decimal from the rs_decimal crate root.

```rust
pub use decimal::Decimal as Decimal
```

## Normal usage and semantics

Use rs_decimal::Decimal; this is the public import path for the same item, not an additional numerical implementation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-decimal" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
