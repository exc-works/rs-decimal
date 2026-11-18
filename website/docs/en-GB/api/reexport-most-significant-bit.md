---
locale: en-GB
source_revision: fe0a2cbd7e86aa455f6274096dc78e80786ab081e8fd78826b22f34970a7dfa9
translation_provenance: en-US-reviewed-reuse
api_id: reexport::most_significant_bit
---
# reexport::most_significant_bit

Import most_significant_bit from the crate root.

```rust
pub use conversion::most_significant_bit as most_significant_bit
```

## Parameters, return and normal usage

This is the public Rust reexport, not a new type or implementation. The import syntax is tested natively. Browser equivalents are bigint, an exact rational field object and the global most_significant_bit function respectively.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-most-significant-bit" locale="en-GB" />

## Browser boundaries

Integer transport does not pass through lossy JS numbers. Conversion tuples retain Rust value/flag order; false does not promise an exact result. Borrowed inputs remain unchanged and constructors initialize results. The [resource policy](../resource-policy) bounds integer inputs, output scale and rational coefficient growth separately from library errors. Rust import syntax is explicitly labeled as native-tested.

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
