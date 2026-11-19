---
locale: en-US
source_revision: 24d7fa4a2221c5f03dbc4893af4fe9b8c3b42496692a1b9ed6c8a03465745caf
api_id: reexport::max
---
# reexport::max

Import max from the rs_decimal crate root.

```rust
pub use decimal::max as max
```

## Normal usage and semantics

Use rs_decimal::max; this is the public import path for the same item, not an additional numerical implementation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-max" locale="en-US" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
