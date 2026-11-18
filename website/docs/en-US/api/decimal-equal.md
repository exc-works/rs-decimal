---
locale: en-US
source_revision: 6d86e4a4d53be90de2e3c79ca0231980d419a71194ef67f300c06ca432f017c2
api_id: Decimal::equal
---
# Decimal::equal

Test whether two decimals represent the same number.

```rust
fn equal (& self , other : & Self) -> bool
```

## Parameters, return and usage notes

Borrow other: &Decimal; return bool/boolean. Scale and initialization state are ignored. JavaScript === compares wrapper identity; call equal() for Rust numerical equality.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-equal" locale="en-US" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
