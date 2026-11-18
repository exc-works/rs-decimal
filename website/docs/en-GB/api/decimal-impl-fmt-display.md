---
locale: en-GB
source_revision: c94998594bf6a8005b50df98354b3549d47447f21a6664735b824dda07443935
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::impl fmt :: Display"
---
# Decimal::impl fmt :: Display

Render a decimal with Rust formatting syntax.

```rust
impl fmt :: Display for Decimal
```

## Normal usage and semantics

Use format!("{}", value), println! or to_string(); fmt::Formatter is supplied by the formatting machinery. Default formatting trims fractional trailing zeros. Additional formatting flags are covered in the formatting group; use string()/string_with_trailing_zeros() in the bridge.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fmt-display" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
