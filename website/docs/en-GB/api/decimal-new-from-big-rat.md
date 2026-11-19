---
locale: en-GB
source_revision: 609e506e194d4140e441e114e8a66b7a118a2cb443c002b71a5be2c8260307ad
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="conversion-decimal-new-from-big-rat" locale="en-GB" />

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
