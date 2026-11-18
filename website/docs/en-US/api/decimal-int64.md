---
locale: en-US
source_revision: 91bb241ee917c58402441fd00f901b8aa75a693fa5e9137596c74ca66e946c7e
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

<MethodExample id="conversion-decimal-int64" locale="en-US" />

## Browser boundaries

Integer transport does not pass through lossy JS numbers. Conversion tuples retain Rust value/flag order; false does not promise an exact result. Borrowed inputs remain unchanged and constructors initialize results. The [resource policy](../resource-policy) bounds integer inputs, output scale and rational coefficient growth separately from library errors. Rust import syntax is explicitly labeled as native-tested.

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
