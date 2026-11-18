---
locale: en-GB
source_revision: 05a4d32c73f6c4d59c8e657272c093b1db00da738b0234c960526209d3e3196c
translation_provenance: en-US-reviewed-reuse
api_id: reexport::NullDecimal
---
# reexport::NullDecimal

Import NullDecimal from the rs_decimal crate root.

```rust
pub use decimal::NullDecimal as NullDecimal
```

## Normal usage and semantics

Use rs_decimal::NullDecimal; this is the public import path for the same item, not an additional numerical implementation. JavaScript has no Rust use syntax; see the corresponding bridge reference.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-nulldecimal" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
