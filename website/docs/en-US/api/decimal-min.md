---
locale: en-US
source_revision: eafc21e4a88e29bea4168f39bfe3b5db899f2a28bccbd47f88733ffdaab27a0c
api_id: Decimal::min
---
# Decimal::min

Copy the smaller of two numerical values.

```rust
fn min (& self , other : & Self) -> Self
```

## Parameters, return and usage notes

Borrow other: &Decimal; return an independent Decimal. A tie preserves the first operand’s scale and initialization state. Use Decimal::min(&left, &right) in Rust to avoid selecting the consuming Ord::min trait method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-min" locale="en-US" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
