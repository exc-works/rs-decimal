---
locale: en-US
source_revision: 4d2afeddcd523e94b9e9276fe6f8c19193bfe95254c1ef9ded67b301a566152b
api_id: Decimal::min_ref
---
# Decimal::min_ref

Select an original operand by numerical comparison.

```rust
fn min_ref < 'a > (& 'a self , other : & 'a Self) -> & 'a Self
```

## Parameters, return and normal usage

other: &Decimal; Rust returns a shared reference with the input lifetime. Equal values select self, preserving scale and initialization. The bridge returns the corresponding original wrapper, not a copy. Moving/freeing that wrapper invalidates every JS alias to it; JS cannot enforce Rust borrow lifetimes. Use max/min for an independent owned result.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-min-ref" locale="en-US" />

## Browser boundaries

Follow the ownership rules above. Access after a move returns a moved error; free is idempotent. Type and alias validation precede a move, but Rust errors do not restore the consumed receiver. The [resource policy](../resource-policy) is separate from library errors: powers check intermediate values and alignment checks coefficient growth. Conservative checks can reject small final results.

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
