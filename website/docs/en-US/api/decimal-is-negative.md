---
locale: en-US
source_revision: 9ffad200b8eee0be119e53ec6189311fb6b54cada31774d04149bdc2b868ac4e
api_id: Decimal::is_negative
---
# Decimal::is_negative

Test whether the numerical value is strictly negative.

```rust
fn is_negative (& self) -> bool
```

## Parameters, return and usage notes

Returns bool/boolean. Zero, including an uninitialized default, returns false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-negative" locale="en-US" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
