---
locale: en-US
source_revision: 8e2ccd4d5de92023ccad23e3b1ce732438fad5dab0ed6652057ad04ade8ca4a5
api_id: reexport::hundred
---
# reexport::hundred

Import hundred from the rs_decimal crate root.

```rust
pub use decimal::hundred as hundred
```

## Normal usage and semantics

Use rs_decimal::hundred; this is the public import path for the same item, not an additional numerical implementation. JavaScript has no Rust use syntax; see the corresponding bridge reference.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-hundred" locale="en-US" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
