---
locale: en-US
source_revision: b417c8ff7bf7c7086d2d314348cd490ed506bea3f1d87b65cb817d90defb82d7
api_id: Decimal::big_rat
---
# Decimal::big_rat

Read the exact reduced rational value.

```rust
fn big_rat (& self) -> BigRational
```

## Parameters, return and normal usage

No parameters; Rust returns BigRational. The bridge returns an immutable {numerator: bigint, denominator: bigint} snapshot, normalized with a positive denominator. Decimal scale and trailing zeros do not survive as rational metadata; zero becomes 0/1. The input is borrowed and unchanged.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-big-rat" locale="en-US" />

## Browser boundaries

Integer transport does not pass through lossy JS numbers. Conversion tuples retain Rust value/flag order; false does not promise an exact result. Borrowed inputs remain unchanged and constructors initialize results. The [resource policy](../resource-policy) bounds integer inputs, output scale and rational coefficient growth separately from library errors. Rust import syntax is explicitly labeled as native-tested.

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
