---
locale: en-GB
source_revision: 52eca22e980279ceae03e771dce435ca3ba3fd74042d6f5541b41c478a23c74f
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::max_ref
---
# Decimal::max_ref

Select an original operand by numerical comparison.

```rust
fn max_ref < 'a > (& 'a self , other : & 'a Self) -> & 'a Self
```

## Parameters, return and normal usage

other: &Decimal; Rust returns a shared reference with the input lifetime. Equal values select self, preserving scale and initialization. The bridge returns the corresponding original wrapper, not a copy. Moving/freeing that wrapper invalidates every JS alias to it; JS cannot enforce Rust borrow lifetimes. Use max/min for an independent owned result.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-max-ref" locale="en-GB" />

## Related methods

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
