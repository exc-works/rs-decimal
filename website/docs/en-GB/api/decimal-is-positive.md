---
locale: en-GB
source_revision: 775716427f3737c65dee2b46317125f81d53dce3779b9de7255849e290c2308b
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::is_positive
---
# Decimal::is_positive

Test whether the numerical value is strictly positive.

```rust
fn is_positive (& self) -> bool
```

## Parameters, return and usage notes

Returns bool/boolean. Zero, including an uninitialized default, returns false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-positive" locale="en-GB" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
