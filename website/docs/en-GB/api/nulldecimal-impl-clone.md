---
locale: en-GB
source_revision: 056afbd5297b8421283bf4270ff73e28ea9129becd2598b31ff7a84ee04dacd5
translation_provenance: en-US-reviewed-reuse
api_id: NullDecimal::impl Clone
---
# NullDecimal::impl Clone

Clone both validity and the stored decimal in Rust.

```rust
impl Clone for NullDecimal
```

## Normal usage and semantics

Cloning preserves hidden values even when valid is false. This is a native Rust trait; it is not a live shared reference.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-clone" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
