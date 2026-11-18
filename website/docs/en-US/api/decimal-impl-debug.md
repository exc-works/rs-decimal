---
locale: en-US
source_revision: 150ae4511881de6cce3960793291dd735b00fc7aa5175f86d5957a93c70d0c13
api_id: Decimal::impl Debug
---
# Decimal::impl Debug

Inspect a decimal during Rust debugging.

```rust
impl Debug for Decimal
```

## Normal usage and semantics

The derived diagnostic representation includes internal fields; it is not a stable serialization format. Browser output uses explicit snapshots instead.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-debug" locale="en-US" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
