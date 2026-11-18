---
locale: en-GB
source_revision: 3ced5d99beade676205267c80e93f90c49f974486393990035a12b18f4061c4a
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::impl PartialEq
---
# Decimal::impl PartialEq

Compare numerical equality using Rust operators or PartialEq.

```rust
impl PartialEq for Decimal
```

## Normal usage and semantics

eq borrows both operands and returns bool. 12.00 == 12.0. JavaScript uses equal(); === tests wrapper identity.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialeq" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
