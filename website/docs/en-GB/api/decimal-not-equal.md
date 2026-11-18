---
locale: en-GB
source_revision: 7f94821cf570d2860956d25bb69a5d1cdbe5a253fac837b02903b87236f85c19
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::not_equal
---
# Decimal::not_equal

Test whether two decimals represent different numbers.

```rust
fn not_equal (& self , other : & Self) -> bool
```

## Parameters, return and usage notes

Borrow other: &Decimal; return bool/boolean. This is the inverse of equal(), not a comparison of coefficient/scale snapshots.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-not-equal" locale="en-GB" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
