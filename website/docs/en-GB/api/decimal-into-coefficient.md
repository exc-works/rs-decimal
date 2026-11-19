---
locale: en-GB
source_revision: 784a138fd9dfff6c4bbabb3b837f6e63c10c904c5578943fe043993afcf99a55
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="owned-decimal-into-coefficient" locale="en-GB" />

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
