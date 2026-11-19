---
locale: en-US
source_revision: 181ceff066af430ef0b4f061e0d4890d14b967a20ab1d2fe96ac03e3b4f024fe
api_id: Decimal::add_owned
---
# Decimal::add_owned

Consume the left operand for exact arithmetic.

```rust
fn add_owned (self , other : & Self) -> Self
```

## Parameters, return and normal usage

other: &Decimal; returns an initialized Decimal at the larger input scale. The receiver moves, while other remains usable. Storage can be reused, but growth can allocate. The JavaScript API rejects the same wrapper as both receiver and other before moving it. Once Rust is entered, even a resource error consumes the receiver.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-add-owned" locale="en-US" />

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
