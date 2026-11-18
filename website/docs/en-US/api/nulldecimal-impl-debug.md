---
locale: en-US
source_revision: 4b0752061c959ac5ce6e3b7af4bf7c9232d0d1cfb43869372b4caa7a5d6f49f4
api_id: NullDecimal::impl Debug
---
# NullDecimal::impl Debug

Inspect nullable state with Rust Debug.

```rust
impl Debug for NullDecimal
```

## Normal usage and semantics

Diagnostic text contains validity and stored fields, including hidden values. It is not a stable wire format; the browser uses snapshots.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-debug" locale="en-US" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
