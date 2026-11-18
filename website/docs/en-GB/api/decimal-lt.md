---
locale: en-GB
source_revision: fe8ae2df48c06b20c3ac1c7bbd224d12734d4193d035a1a96ed7e524e49fd64b
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::lt
---
# Decimal::lt

Test whether this value is less than another.

```rust
fn lt (& self , other : & Self) -> bool
```

## Parameters, return and usage notes

Borrow other: &Decimal; return bool/boolean. Equal numerical values return false even if their stored scales differ.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lt" locale="en-GB" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
