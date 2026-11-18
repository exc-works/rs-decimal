---
locale: en-GB
source_revision: 17e780c5fcd60ccd7701083f059fdd79ff3607aa9dff1b3dfa127b396f366781
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::new_from_float32
---
# Decimal::new_from_float32

Construct from the shortest decimal that round-trips to a float.

```rust
fn new_from_float32 (value : f32) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

value: f32, passed as a JS number. The bridge first narrows binary64 to binary32, so 16777217 becomes 16777216. This does not recover decimal digits already lost to binary floating point. Use new_from_string for exact decimal input. NaN and either infinity return InvalidFormat; finite f64 that overflows during f32 narrowing also returns InvalidFormat. Negative zero becomes initialized Decimal zero without a sign.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float32" locale="en-GB" />

## Browser boundaries

Integer transport does not pass through lossy JS numbers. Conversion tuples retain Rust value/flag order; false does not promise an exact result. Borrowed inputs remain unchanged and constructors initialize results. The [resource policy](../resource-policy) bounds integer inputs, output scale and rational coefficient growth separately from library errors. Rust import syntax is explicitly labeled as native-tested.

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
