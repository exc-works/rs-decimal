---
locale: en-GB
source_revision: 1cd08b3844b9339373458bc008707757ca17e7772da4c888bc88a31f9efef788
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::min
---
# Decimal::min

Copy the smaller of two numerical values.

```rust
fn min (& self , other : & Self) -> Self
```

## Parameters, return and usage notes

Borrow other: &Decimal; return an independent Decimal. A tie preserves the first operand’s scale and initialization state. Use Decimal::min(&left, &right) in Rust to avoid selecting the consuming Ord::min trait method.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-min" locale="en-GB" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
