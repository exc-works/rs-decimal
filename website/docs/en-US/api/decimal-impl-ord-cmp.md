---
locale: en-US
source_revision: 317e1f4c19916312bb2823fc0a526f5311afada3dc44c8d3247bdd5fc0f08111
api_id: Decimal::impl Ord::cmp
---
# Decimal::impl Ord::cmp

Order decimals numerically with Rust Ord.

```rust
fn cmp (& self , other : & Self) -> Ordering
```

## Normal usage and semantics

Ord::cmp returns Ordering, unlike Decimal::cmp which returns -1/0/1. Representation differences do not affect ordering.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-ord-cmp" locale="en-US" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
