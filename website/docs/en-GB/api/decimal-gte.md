---
locale: en-GB
source_revision: ecdf018f489484305dd62f934b29d0a6759b6939d7abb56d9717dd4b72e36899
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::gte
---
# Decimal::gte

Test whether this value is greater than or equal to another.

```rust
fn gte (& self , other : & Self) -> bool
```

## Parameters, return and usage notes

Borrow other: &Decimal; return bool/boolean. Equality is numerical, independent of scale and initialization state.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-gte" locale="en-GB" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
