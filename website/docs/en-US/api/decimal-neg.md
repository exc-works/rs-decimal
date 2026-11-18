---
locale: en-US
source_revision: 16b33be7c90300f9c4b236f60591e3b54c1078c3a213fb54df0f571ca8f60e09
api_id: Decimal::neg
---
# Decimal::neg

Create an independent decimal with the sign reversed.

```rust
fn neg (& self) -> Self
```

## Parameters, return and usage notes

Returns Decimal and preserves scale and trailing zeros. Zero stays numerically zero; the result is initialized even for a default input.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-neg" locale="en-US" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
