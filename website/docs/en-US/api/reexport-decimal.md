---
locale: en-US
source_revision: 893ae85fdbe2ce4fb0a97cdee41fd9675ed135240e7df9b22b449b50279fb13c
api_id: reexport::Decimal
---
# reexport::Decimal

Import Decimal from the rs_decimal crate root.

```rust
pub use decimal::Decimal as Decimal
```

## Normal usage and semantics

Use rs_decimal::Decimal; this is the public import path for the same item, not an additional numerical implementation. JavaScript has no Rust use syntax; see the corresponding bridge reference.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-decimal" locale="en-US" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
