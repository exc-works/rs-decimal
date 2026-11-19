---
locale: en-GB
source_revision: 03df94c2d70888cf880122deea0e6ec382870fa1e10b3d6cb2d26b3fc0fb348a
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::gte
---
# Decimal::gte

Test whether this value is greater than or equal to another.

```rust
fn gte (& self , other : & Self) -> bool
```

## Parameters, return and usage notes

Borrow other: &Decimal; return bool/boolean. Equality is numerical, independent of scale and initialization state.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-gte" locale="en-GB" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
