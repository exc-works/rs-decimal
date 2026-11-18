---
locale: en-US
source_revision: 73008a8984515f08334f6194e58d2ac82c8d4e900805ae3c45e10f94eb7747b8
api_id: Decimal::gt
---
# Decimal::gt

Test whether this value is greater than another.

```rust
fn gt (& self , other : & Self) -> bool
```

## Parameters, return and usage notes

Borrow other: &Decimal; return bool/boolean. Equal numerical values return false even if their stored scales differ.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-gt" locale="en-US" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
