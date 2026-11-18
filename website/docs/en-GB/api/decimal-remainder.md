---
locale: en-GB
source_revision: d4492d82b410cc5d7b67274496227b1fa1d6279b6d276d6bd8bfbe25ef422dd4
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::remainder
---
# Decimal::remainder

Split the integer part and unscaled fractional remainder.

```rust
fn remainder (& self) -> (BigInt , BigInt)
```

## Parameters, return and normal usage

No parameters; returns (BigInt, BigInt), mapped to [bigint, bigint]. The first value truncates toward zero; the second is a signed coefficient remainder, not a Decimal fraction. For -12.750 the result is (-12, -750): the second value needs the original scale 3 to mean -0.750. Default zero returns (0, 0).

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-remainder" locale="en-GB" />

## Browser boundaries

Follow the ownership rules above. Access after a move returns a moved error; free is idempotent. Type and alias validation precede a move, but Rust errors do not restore the consumed receiver. The [resource policy](../resource-policy) is separate from library errors: powers check intermediate values and alignment checks coefficient growth. Conservative checks can reject small final results.

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
