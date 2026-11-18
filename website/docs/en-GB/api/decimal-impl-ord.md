---
locale: en-GB
source_revision: fc360056fa6a06d0df76e11b3c2a65adb20ebcb5dcc14e8a0ad1eefc1447a90e
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::impl Ord
---
# Decimal::impl Ord

Order decimals numerically with Rust Ord.

```rust
impl Ord for Decimal
```

## Normal usage and semantics

Ord::cmp returns Ordering, unlike Decimal::cmp which returns -1/0/1. Representation differences do not affect ordering.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-ord" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
