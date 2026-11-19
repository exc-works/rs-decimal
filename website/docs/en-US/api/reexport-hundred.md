---
locale: en-US
source_revision: ef3aa99db04ca1bd7e821c4d4bf0be02f9db3337ae118e60dc96d5695ab892b9
api_id: reexport::hundred
---
# reexport::hundred

Import hundred from the rs_decimal crate root.

```rust
pub use decimal::hundred as hundred
```

## Normal usage and semantics

Use rs_decimal::hundred; this is the public import path for the same item, not an additional numerical implementation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-hundred" locale="en-US" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
