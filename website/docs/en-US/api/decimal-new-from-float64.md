---
locale: en-US
source_revision: fd72fd2afaa0763f177af4f30747e9166deea4da1bb1ba841cbd829b6d791a9d
api_id: Decimal::new_from_float64
---
# Decimal::new_from_float64

Construct from the shortest decimal that round-trips to a float.

```rust
fn new_from_float64 (value : f64) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

value: f64, passed as a JS number. This does not recover decimal digits already lost to binary floating point. Use new_from_string for exact decimal input. NaN and either infinity return InvalidFormat; finite f64 that overflows during f32 narrowing also returns InvalidFormat. Negative zero becomes initialized Decimal zero without a sign.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float64" locale="en-US" />

## Browser boundaries

Integer transport does not pass through lossy JS numbers. Conversion tuples retain Rust value/flag order; false does not promise an exact result. Borrowed inputs remain unchanged and constructors initialize results. The [resource policy](../resource-policy) bounds integer inputs, output scale and rational coefficient growth separately from library errors. Rust import syntax is explicitly labeled as native-tested.

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
