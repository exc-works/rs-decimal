---
locale: en-GB
source_revision: 37fcbc194399d8059373fc9c0c61833d997ffb41871435eb1c6867b7870d1acc
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::impl Clone
---
# Decimal::impl Clone

Clone a decimal while preserving its complete representation.

```rust
impl Clone for Decimal
```

## Normal usage and semantics

Clone copies coefficient, scale and initialization state; JavaScript uses the named clone() adapter rather than a Rust trait.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-clone" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
