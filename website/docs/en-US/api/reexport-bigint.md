---
locale: en-US
source_revision: f07542c46478fd9edd09bbf3af9bd03d15608eb33e9ed99beb51b022df098d6a
api_id: reexport::BigInt
---
# reexport::BigInt

Import BigInt from the crate root.

```rust
pub use num_bigint::BigInt as BigInt
```

## Parameters, return and normal usage

This is the public Rust reexport, not a new type or implementation. The import syntax is tested natively. Browser equivalents are bigint, an exact rational field object and the global most_significant_bit function respectively.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-bigint" locale="en-US" />

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
