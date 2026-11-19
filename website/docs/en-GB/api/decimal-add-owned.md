---
locale: en-GB
source_revision: 851de95326d3b1f10839fcbae723fc6950eca60bcbb6e43b9e406040ff806dc1
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::add_owned
---
# Decimal::add_owned

Consume the left operand for exact arithmetic.

```rust
fn add_owned (self , other : & Self) -> Self
```

## Parameters, return and normal usage

other: &Decimal; returns an initialized Decimal at the larger input scale. The receiver moves, while other remains usable. Storage can be reused, but growth can allocate. The bridge rejects the same wrapper as both receiver and other before moving it. Once Rust is entered, even a resource error consumes the receiver.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-add-owned" locale="en-GB" />

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
