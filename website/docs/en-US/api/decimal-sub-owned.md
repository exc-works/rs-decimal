---
locale: en-US
source_revision: dfbad807bf85fc60fbbfb720800bbf36eb483a86897e4245a66f95ff012fb636
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

## Browser boundaries

Follow the ownership rules above. Access after a move returns a moved error; free is idempotent. Type and alias validation precede a move, but Rust errors do not restore the consumed receiver. The [resource policy](../resource-policy) is separate from library errors: powers check intermediate values and alignment checks coefficient growth. Conservative checks can reject small final results.

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
