---
locale: en-US
source_revision: 3d99b7dec25de936ed85161e3eb8cf4565f56b88649e6c879a6468d051308404
api_id: Decimal::new_from_big_rat_with_prec
---
# Decimal::new_from_big_rat_with_prec

Round a rational to an explicit number of decimal places.

```rust
fn new_from_big_rat_with_prec (value : & BigRational , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

value: &BigRational; prec: nonnegative isize; mode: RoundingMode. The JavaScript API uses an exact numerator/denominator bigint object. Unlike exact conversion, nonterminating fractions such as 1/3 are allowed. Negative precision returns InvalidPrecision; required Unnecessary rounding returns RoundUnnecessary. Zero denominator is invalid input before calling Rust; zero numerator returns initialized zero at prec.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat-with-prec" locale="en-US" />

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
