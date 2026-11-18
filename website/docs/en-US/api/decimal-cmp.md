---
locale: en-US
source_revision: 19a39638a4734aeba49de6f5af4a9f79e89b842f7b78ba2cf7aeb511f0ebcd6c
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

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
