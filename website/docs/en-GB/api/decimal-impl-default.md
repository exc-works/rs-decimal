---
locale: en-GB
source_revision: 9ab9d90f4fdbf97aeb5b036e4b2c138345a66dab28af2a920054edaddf0608dd
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::impl Default
---
# Decimal::impl Default

Create an uninitialized numerical zero with Rust Default.

```rust
impl Default for Decimal
```

## Normal usage and semantics

Default is numerically equal to constructed zero but is_nil() is true. The bridge offers Decimal.default(); this page describes the Rust trait itself.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-default" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
