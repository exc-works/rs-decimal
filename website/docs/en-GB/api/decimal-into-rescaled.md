---
locale: en-GB
source_revision: f56b3cc9eb8667e7cf067d7d1256d88fafb84d3cb5b89748b59c7df428bab4cf
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

## Browser boundaries

Follow the ownership rules above. Access after a move returns a moved error; free is idempotent. Type and alias validation precede a move, but Rust errors do not restore the consumed receiver. The [resource policy](../resource-policy) is separate from library errors: powers check intermediate values and alignment checks coefficient growth. Conservative checks can reject small final results.

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
