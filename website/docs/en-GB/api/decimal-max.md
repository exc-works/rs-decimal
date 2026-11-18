---
locale: en-GB
source_revision: 5956431328357316a87f05c2d1543bb100b12923bb2d50b1e21ff3f96212db45
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::max
---
# Decimal::max

Copy the larger of two numerical values.

```rust
fn max (& self , other : & Self) -> Self
```

## Parameters, return and usage notes

Borrow other: &Decimal; return an independent Decimal. A tie preserves the first operand’s scale and initialization state. Use Decimal::max(&left, &right) in Rust to select this borrowed inherent method explicitly; Ord::max takes ownership.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-max" locale="en-GB" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
