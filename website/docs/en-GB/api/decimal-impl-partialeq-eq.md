---
locale: en-GB
source_revision: 1d252fa81f50f2414d333721b6e7df59a5331a4689705908696e9896f6fb4dbf
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::impl PartialEq::eq
---
# Decimal::impl PartialEq::eq

Compare numerical equality using Rust operators or PartialEq.

```rust
fn eq (& self , other : & Self) -> bool
```

## Normal usage and semantics

eq borrows both operands and returns bool. 12.00 == 12.0. JavaScript uses equal(); === tests wrapper identity.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialeq-eq" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
