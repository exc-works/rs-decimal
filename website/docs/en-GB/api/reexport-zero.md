---
locale: en-GB
source_revision: 099c3533a52ede06adc92af14955d2d2a20dadf66c185c385b9eda728b52b6ba
translation_provenance: en-US-reviewed-reuse
api_id: reexport::zero
---
# reexport::zero

Import zero from the rs_decimal crate root.

```rust
pub use decimal::zero as zero
```

## Normal usage and semantics

Use rs_decimal::zero; this is the public import path for the same item, not an additional numerical implementation. JavaScript has no Rust use syntax; see the corresponding bridge reference.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-zero" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
