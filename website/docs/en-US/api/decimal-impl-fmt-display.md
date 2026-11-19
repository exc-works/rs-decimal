---
locale: en-US
source_revision: 97b85000fbf7a0fb6537eae68258f927a7739647e41e96de3c0f26f460f2d35a
api_id: "Decimal::impl fmt :: Display"
---
# Decimal::impl fmt :: Display

Render a decimal with Rust formatting syntax.

```rust
impl fmt :: Display for Decimal
```

## Normal usage and semantics

Use format!("{}", value), println! or to_string(); fmt::Formatter is supplied by the formatting machinery. Default formatting trims fractional trailing zeros. Additional formatting flags are covered in the formatting group; use string()/string_with_trailing_zeros() in JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fmt-display" locale="en-US" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
