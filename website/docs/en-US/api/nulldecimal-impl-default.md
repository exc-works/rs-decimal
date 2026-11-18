---
locale: en-US
source_revision: 71bc56362fd9f1c25a96932ba1ffd795db889517f58a951726bcd78e58fe2bc7
api_id: NullDecimal::impl Default
---
# NullDecimal::impl Default

Create an absent nullable using Rust Default.

```rust
impl Default for NullDecimal
```

## Normal usage and semantics

Both validity and inner initialization are false. The bridge provides NullDecimal.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-default" locale="en-US" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
