---
locale: en-US
source_revision: 8110d2fd30808fd737c3f5730a5be2d19cf89f407f5d688859de476689342458
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

## Browser boundaries

Integer transport does not pass through lossy JS numbers. Conversion tuples retain Rust value/flag order; false does not promise an exact result. Borrowed inputs remain unchanged and constructors initialize results. The [resource policy](../resource-policy) bounds integer inputs, output scale and rational coefficient growth separately from library errors. Rust import syntax is explicitly labeled as native-tested.

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
