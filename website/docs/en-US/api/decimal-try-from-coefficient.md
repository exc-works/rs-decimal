---
locale: en-US
source_revision: 34aa17297c2ce6b5b73e997c910f57874da73c70f9cf52cd3fadd2ab710a529d
api_id: Decimal::try_from_coefficient
---
# Decimal::try_from_coefficient

Construct an initialized value from an owned coefficient and scale.

```rust
fn try_from_coefficient (coefficient : BigInt , scale : usize) -> Result < Self , DecimalError >
```

## Parameters, return and normal usage

coefficient: BigInt; scale: usize (nonnegative decimal places). Retains trailing zeros and scale for zero; it does not use text-parser normalization. Rust returns Overflow above isize::MAX. The browser uses exact bigint plus an integer number in wasm32 usize range; scales above i32::MAX retain Overflow, and other scales above 1000 exceed the playground resource budget. JS bigint is immutable, so it is not a moved wrapper.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-try-from-coefficient" locale="en-US" />

## Browser boundaries

Follow the ownership rules above. Access after a move returns a moved error; free is idempotent. Type and alias validation precede a move, but Rust errors do not restore the consumed receiver. The [resource policy](../resource-policy) is separate from library errors: powers check intermediate values and alignment checks coefficient growth. Conservative checks can reject small final results.

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
