---
locale: en-US
source_revision: 3e32f687004e72286fa606992f7977ccbc210f3d36756c6006ab536cd5091f5a
api_id: Decimal::new_from_decimal
---
# Decimal::new_from_decimal

Copy an existing Decimal, including its stored representation and initialization state.

```rust
fn new_from_decimal (value : & Self) -> Self
```

## Parameters and return

`value: &Decimal` is borrowed. The returned Decimal is independent; the input remains usable.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-decimal" locale="en-US" />

## Usage notes

Copying `Decimal::default()` preserves uninitialized state; it does not turn it into initialized zero. In JavaScript, pass a live Decimal from the same run. This constructor does not consume it.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
