---
locale: en-GB
source_revision: 60ca5295cf77ea0e6cccdadb5d937c5adbdd69051a5dbac42387fec9ee90828c
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::lte
---
# Decimal::lte

Test whether this value is less than or equal to another.

```rust
fn lte (& self , other : & Self) -> bool
```

## Parameters, return and usage notes

Borrow other: &Decimal; return bool/boolean. Equality is numerical, independent of scale and initialization state.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lte" locale="en-GB" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
