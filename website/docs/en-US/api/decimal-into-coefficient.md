---
locale: en-US
source_revision: cc52467a55bcaa7e86e52d9cf523d5b8a22924e126619052d59e51cd0b6aab8b
api_id: Decimal::into_coefficient
---
# Decimal::into_coefficient

Move out the unscaled integer coefficient.

```rust
fn into_coefficient (self) -> BigInt
```

## Parameters, return and normal usage

No parameters; returns BigInt in Rust and bigint in JS. Scale and initialization state are discarded, so 1.2300 yields 12300. Default zero yields 0. The Decimal is consumed and cannot be read again; use coefficient for a non-consuming snapshot in JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-coefficient" locale="en-US" />

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
