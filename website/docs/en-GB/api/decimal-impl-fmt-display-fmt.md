---
locale: en-GB
source_revision: ed7eae766562aad92ba36a1a91ecd3051f3e73486fdc4f475516ea974adac927
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::impl fmt :: Display::fmt"
---
# Decimal::impl fmt :: Display::fmt

Render a decimal with Rust formatting syntax.

```rust
fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result
```

## Normal usage and semantics

Use format!("{}", value), println! or to_string(); fmt::Formatter is supplied by the formatting machinery. Default formatting trims fractional trailing zeros. Additional formatting flags are covered in the formatting group; use string()/string_with_trailing_zeros() in the bridge.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fmt-display-fmt" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
