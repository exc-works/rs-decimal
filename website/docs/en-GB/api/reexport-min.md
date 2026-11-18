---
locale: en-GB
source_revision: 0d0d53ee31abc5089791404b98708465e5ae1271ee54c709858dd761b9a5ca22
translation_provenance: en-US-reviewed-reuse
api_id: reexport::min
---
# reexport::min

Import min from the rs_decimal crate root.

```rust
pub use decimal::min as min
```

## Normal usage and semantics

Use rs_decimal::min; this is the public import path for the same item, not an additional numerical implementation. JavaScript has no Rust use syntax; see the corresponding bridge reference.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-min" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
