---
locale: en-US
source_revision: 098d219e5e0932c9734847caf4da4f02238ac75ca2e27400d9527165d61283dd
api_id: Decimal::cmp
---
# Decimal::cmp

Compare two numerical values with a three-way result.

```rust
fn cmp (& self , other : & Self) -> i32
```

## Parameters, return and usage notes

Borrow other: &Decimal (Decimal in JavaScript). Returns i32/number: -1, 0 or 1. Different scales and initialization states do not affect numeric equality. This is the inherent method, not Ord::cmp returning Ordering.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-cmp" locale="en-US" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
