---
locale: en-GB
source_revision: 0db8fa19358a4f2f8842ab2f8da65e8e108f7914cbdecf8f41f654ca5171b70c
translation_provenance: en-US-reviewed-reuse
api_id: reexport::max
---
# reexport::max

Import max from the rs_decimal crate root.

```rust
pub use decimal::max as max
```

## Normal usage and semantics

Use rs_decimal::max; this is the public import path for the same item, not an additional numerical implementation. JavaScript has no Rust use syntax; see the corresponding bridge reference.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-max" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
