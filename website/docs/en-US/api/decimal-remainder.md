---
locale: en-US
source_revision: 64d94a480e185cbb38c975ef60712d5ae4ac373961962b675392db6ef9375f09
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

<MethodExample id="owned-decimal-remainder" locale="en-US" />

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
