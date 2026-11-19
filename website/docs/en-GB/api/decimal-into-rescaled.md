---
locale: en-GB
source_revision: aed80a094ad154f3abfbbdc72125d175f579082056ef3b224d8dda331e64cff5
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::into_rescaled
---
# Decimal::into_rescaled

Consume a decimal while changing its decimal places.

```rust
fn into_rescaled (self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

prec: isize; mode: RoundingMode. Like rescale, this rounds or appends zeros and always initializes the result. The receiver is moved even when Rust returns InvalidPrecision, RoundUnnecessary or a browser resource error. Invalid JS shape/range is rejected before entering Rust and does not consume it.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-rescaled" locale="en-GB" />

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
