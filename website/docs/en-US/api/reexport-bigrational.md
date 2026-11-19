---
locale: en-US
source_revision: f67e1caa0fb769c278b4d8058441646a0cf632963d9da4fd8b19596b64289dc8
api_id: reexport::BigRational
---
# reexport::BigRational

Import BigRational from the crate root.

```rust
pub use num_rational::BigRational as BigRational
```

## Parameters, return and normal usage

This is the public Rust reexport, not a new type or implementation. The import syntax is tested natively. Browser equivalents are bigint, an exact rational field object and the global most_significant_bit function respectively.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-bigrational" locale="en-US" />

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
