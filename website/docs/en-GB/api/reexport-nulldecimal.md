---
locale: en-GB
source_revision: f04757565931a8e8fa648872ee9153875f735f45247f70875bf4fa04dbb2585d
translation_provenance: en-US-reviewed-reuse
api_id: reexport::NullDecimal
---
# reexport::NullDecimal

Import NullDecimal from the rs_decimal crate root.

```rust
pub use decimal::NullDecimal as NullDecimal
```

## Normal usage and semantics

Use rs_decimal::NullDecimal; this is the public import path for the same item, not an additional numerical implementation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-nulldecimal" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
