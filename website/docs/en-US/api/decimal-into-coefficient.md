---
locale: en-US
source_revision: fa1a3656d76ca2759cc49aed5b71589136f0ed7e84a57f811be7062ea9aa8860
api_id: Decimal::into_coefficient
---
# Decimal::into_coefficient

Move out the unscaled integer coefficient.

```rust
fn into_coefficient (self) -> BigInt
```

## Parameters, return and normal usage

No parameters; returns BigInt in Rust and bigint in JS. Scale and initialization state are discarded, so 1.2300 yields 12300. Default zero yields 0. The Decimal is consumed and cannot be read again; use coefficient for a non-consuming snapshot in the bridge.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-coefficient" locale="en-US" />

## Browser boundaries

Follow the ownership rules above. Access after a move returns a moved error; free is idempotent. Type and alias validation precede a move, but Rust errors do not restore the consumed receiver. The [resource policy](../resource-policy) is separate from library errors: powers check intermediate values and alignment checks coefficient growth. Conservative checks can reject small final results.

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
