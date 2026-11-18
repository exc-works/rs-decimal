---
locale: en-GB
source_revision: 309b64e5487e4ac1d2ea8cf88852216653b617cb1ed53a7d6bdd1a0dffa2d494
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::uint64
---
# Decimal::uint64

Convert to a fixed-width integer with a success flag.

```rust
fn uint64 (& self) -> (u64 , bool)
```

## Parameters, return and normal usage

No parameters; returns (u64, bool), mapped to [bigint, boolean]. Integral in-range values return the integer and true. Fractions and overflow return (0, false), not a truncated value or an exception. Negative values also return (0, false). Scale alone does not determine integrality: 12.00 succeeds. Always inspect the flag to distinguish failure from a valid zero.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-uint64" locale="en-GB" />

## Browser boundaries

Integer transport does not pass through lossy JS numbers. Conversion tuples retain Rust value/flag order; false does not promise an exact result. Borrowed inputs remain unchanged and constructors initialize results. The [resource policy](../resource-policy) bounds integer inputs, output scale and rational coefficient growth separately from library errors. Rust import syntax is explicitly labeled as native-tested.

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
