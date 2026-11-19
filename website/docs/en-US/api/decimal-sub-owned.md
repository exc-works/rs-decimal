---
locale: en-US
source_revision: 347ad719aa4b33bd925c7dfe63b5e557c57b390b4d8d0b57d7aff0d9ca62ac93
api_id: Decimal::sub_owned
---
# Decimal::sub_owned

Consume the left operand for exact arithmetic.

```rust
fn sub_owned (self , other : & Self) -> Self
```

## Parameters, return and normal usage

other: &Decimal; returns an initialized Decimal at the larger input scale. The receiver moves, while other remains usable. Storage can be reused, but growth can allocate. The JavaScript API rejects the same wrapper as both receiver and other before moving it. Once Rust is entered, even a resource error consumes the receiver.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-sub-owned" locale="en-US" />

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
