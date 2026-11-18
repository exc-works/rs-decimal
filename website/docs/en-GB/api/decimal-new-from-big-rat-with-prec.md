---
locale: en-GB
source_revision: 75750ddb09ce29d94b77a840b238d39bf569cfb662c01d997d14bb79d57564c3
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::new_from_big_rat_with_prec
---
# Decimal::new_from_big_rat_with_prec

Round a rational to an explicit number of decimal places.

```rust
fn new_from_big_rat_with_prec (value : & BigRational , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

value: &BigRational; prec: nonnegative isize; mode: RoundingMode. The bridge uses an exact numerator/denominator bigint object. Unlike exact conversion, nonterminating fractions such as 1/3 are allowed. Negative precision returns InvalidPrecision; required Unnecessary rounding returns RoundUnnecessary. Zero denominator is invalid bridge input before calling Rust; zero numerator returns initialized zero at prec.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat-with-prec" locale="en-GB" />

## Browser boundaries

Integer transport does not pass through lossy JS numbers. Conversion tuples retain Rust value/flag order; false does not promise an exact result. Borrowed inputs remain unchanged and constructors initialize results. The [resource policy](../resource-policy) bounds integer inputs, output scale and rational coefficient growth separately from library errors. Rust import syntax is explicitly labeled as native-tested.

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
