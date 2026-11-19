---
locale: en-US
source_revision: ddfedf2b2a7b0914238752b884265b0c43b55c647cf39e728c8143e706a5b5ba
api_id: Decimal::sub_owned
---
# Decimal::sub_owned

Consume the left operand for exact arithmetic.

```rust
fn sub_owned (self , other : & Self) -> Self
```

## Parameters, return and normal usage

other: &Decimal; returns an initialized Decimal at the larger input scale. The receiver moves, while other remains usable. Storage can be reused, but growth can allocate. The bridge rejects the same wrapper as both receiver and other before moving it. Once Rust is entered, even a resource error consumes the receiver.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-sub-owned" locale="en-US" />

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
