---
locale: en-US
source_revision: c7909226b96ffde911017d8ead0a3f2edd2a0c894ee744bf3bcbea66574f1bf5
api_id: Decimal::new_from_big_rat
---
# Decimal::new_from_big_rat

Convert a rational to an exact terminating decimal.

```rust
fn new_from_big_rat (value : & BigRational) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

value: &BigRational; the bridge accepts {numerator: bigint, denominator: bigint}. The denominator must be nonzero (bridge parameter error otherwise); Rust BigRational normalizes signs and common factors. Only reduced denominators containing factors 2 and 5 terminate; other factors return InvalidFormat. The result is initialized. Browser checks apply after reduction and bound output scale/coefficient growth before core conversion.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat" locale="en-US" />

## Browser boundaries

Integer transport does not pass through lossy JS numbers. Conversion tuples retain Rust value/flag order; false does not promise an exact result. Borrowed inputs remain unchanged and constructors initialize results. The [resource policy](../resource-policy) bounds integer inputs, output scale and rational coefficient growth separately from library errors. Rust import syntax is explicitly labeled as native-tested.

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
