---
locale: en-US
source_revision: aa32a5479f7baf77d635cc3726288c9b7c2cfa292167bcd9b91265a058234a8b
api_id: Decimal::abs
---
# Decimal::abs

Create an independent decimal containing the absolute value.

```rust
fn abs (& self) -> Self
```

## Parameters, return and usage notes

Returns Decimal and preserves scale and trailing zeros. The result is initialized even when the input is default; the input is unchanged.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-abs" locale="en-US" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
