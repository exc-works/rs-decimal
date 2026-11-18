---
locale: en-US
source_revision: 25b1ec5a4bb0557629fac6d66ca48c015418489e6280f42443d66425e8372440
api_id: Decimal::impl PartialOrd
---
# Decimal::impl PartialOrd

Compare decimals using Rust partial ordering.

```rust
impl PartialOrd for Decimal 
```

## Normal usage and semantics

All Decimal values are comparable, so partial_cmp returns Some(Ordering). Default zero is comparable to constructed zero; scale differences do not produce None.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialord" locale="en-US" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
