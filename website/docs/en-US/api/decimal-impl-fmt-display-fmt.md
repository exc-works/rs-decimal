---
locale: en-US
source_revision: 53c6a8acf9ea21b47dcf7e64e91f78d378746d55bbda43af84954ef21da2900f
api_id: "Decimal::impl fmt :: Display::fmt"
---
# Decimal::impl fmt :: Display::fmt

Render a decimal with Rust formatting syntax.

```rust
fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result
```

## Normal usage and semantics

Use format!("{}", value), println! or to_string(); fmt::Formatter is supplied by the formatting machinery. Default formatting trims fractional trailing zeros. Additional formatting flags are covered in the formatting group; use string()/string_with_trailing_zeros() in JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fmt-display-fmt" locale="en-US" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
