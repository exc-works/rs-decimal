---
locale: en-GB
source_revision: f83d4eab7c44d20bd9f7f112876d8de08078fd38cb0bbaaf09e60fe2891d5d1e
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::int64
---
# Decimal::int64

Convert to a fixed-width integer with a success flag.

```rust
fn int64 (& self) -> (i64 , bool)
```

## Parameters, return and normal usage

No parameters; returns (i64, bool), mapped to [bigint, boolean]. Integral in-range values return the integer and true. Fractions and overflow return (0, false), not a truncated value or an exception. Scale alone does not determine integrality: 12.00 succeeds. Always inspect the flag to distinguish failure from a valid zero.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-int64" locale="en-GB" />

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
