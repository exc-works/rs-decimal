---
locale: en-US
source_revision: 8e571c100355b6f464b7992d5d9d515f9f9923a4538f1e43b8cc48af4b41a4c7
api_id: Decimal::coefficient
---
# Decimal::coefficient

Inspect the exact unscaled integer coefficient.

```rust
fn coefficient (& self) -> & BigInt
```

## Parameters, return and usage notes

Rust borrows &BigInt; JavaScript returns an immutable bigint snapshot. It is not a live reference. The coefficient includes trailing zeros and is not the integer part of the decimal. The example renders bigint as text to preserve all digits in JSON.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-coefficient" locale="en-US" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
