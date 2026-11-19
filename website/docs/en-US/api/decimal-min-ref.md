---
locale: en-US
source_revision: c991ff009b87a451c4d3a01a7e163c4b8e995107fc18ca0a0ef4ec120e3aee71
api_id: Decimal::min_ref
---
# Decimal::min_ref

Select an original operand by numerical comparison.

```rust
fn min_ref < 'a > (& 'a self , other : & 'a Self) -> & 'a Self
```

## Parameters, return and normal usage

other: &Decimal; Rust returns a shared reference with the input lifetime. Equal values select self, preserving scale and initialization. The JavaScript API returns the corresponding original wrapper, not a copy. Moving/freeing that wrapper invalidates every JS alias to it; JS cannot enforce Rust borrow lifetimes. Use max/min for an independent owned result.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-min-ref" locale="en-US" />

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
