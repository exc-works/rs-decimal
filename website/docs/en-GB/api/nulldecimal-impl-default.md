---
locale: en-GB
source_revision: c654c9726a283aeb3b59ec2a73dcd6f45d404c2677f244d97a60cf2ff73a642b
translation_provenance: en-US-reviewed-reuse
api_id: NullDecimal::impl Default
---
# NullDecimal::impl Default

Create an absent nullable using Rust Default.

```rust
impl Default for NullDecimal
```

## Normal usage and semantics

Both validity and inner initialization are false. The JavaScript API provides NullDecimal.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-default" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
