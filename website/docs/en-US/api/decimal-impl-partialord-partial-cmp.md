---
locale: en-US
source_revision: 1a22225e09dfc3b3b1f7504680c33552c2bd9a622eca096fdf946a5ba9c5cfaa
api_id: Decimal::impl PartialOrd::partial_cmp
---
# Decimal::impl PartialOrd::partial_cmp

Compare decimals using Rust partial ordering.

```rust
fn partial_cmp (& self , other : & Self) -> Option < Ordering >
```

## Normal usage and semantics

All Decimal values are comparable, so partial_cmp returns Some(Ordering). Default zero is comparable to constructed zero; scale differences do not produce None.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialord-partial-cmp" locale="en-US" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
