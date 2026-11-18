---
locale: en-GB
source_revision: aaa15879504e10287ba605474fa0c07c65d1d6607caa2a6ac35b407fe6cd46e4
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::is_not_zero
---
# Decimal::is_not_zero

Test whether the numerical value differs from zero.

```rust
fn is_not_zero (& self) -> bool
```

## Parameters, return and usage notes

Returns bool/boolean, the inverse of is_zero(). An uninitialized default returns false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-not-zero" locale="en-GB" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
