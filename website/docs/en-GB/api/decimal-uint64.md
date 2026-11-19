---
locale: en-GB
source_revision: c526a43d69654fb21b49f56d858016f45f88e7f7f6ec8eaa0c13c3e5e939cd03
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

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
