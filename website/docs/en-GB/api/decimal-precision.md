---
locale: en-GB
source_revision: c8a99895a920cbd49f4dba891779055021c167a0addb6791872293807a1937f8
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::precision
---
# Decimal::precision

Read the number of decimal places stored by a value.

```rust
fn precision (& self) -> isize
```

## Parameters, return and usage notes

Returns an isize (integer number in the browser). This is the scale, not the number of significant digits. Trailing zeros contribute to it.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-precision" locale="en-GB" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
