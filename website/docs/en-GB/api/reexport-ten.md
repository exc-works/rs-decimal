---
locale: en-GB
source_revision: 76d9cb7e766229b955c894271a5c238caac503dc592850e7b5cbe4a6a3eb33d3
translation_provenance: en-US-reviewed-reuse
api_id: reexport::ten
---
# reexport::ten

Import ten from the rs_decimal crate root.

```rust
pub use decimal::ten as ten
```

## Normal usage and semantics

Use rs_decimal::ten; this is the public import path for the same item, not an additional numerical implementation. JavaScript has no Rust use syntax; see the corresponding bridge reference.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-ten" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
