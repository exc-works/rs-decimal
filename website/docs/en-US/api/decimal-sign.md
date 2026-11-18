---
locale: en-US
source_revision: 828ccfc55974951d7ced64d269a1959b7a45efe557a7f4202f79dcfc3632ea3f
api_id: Decimal::sign
---
# Decimal::sign

Read the sign as -1, 0, or 1.

```rust
fn sign (& self) -> i32
```

## Parameters, return and usage notes

Returns i32/number. Both initialized and uninitialized zero have sign 0; there is no separate negative-zero sign.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-sign" locale="en-US" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
