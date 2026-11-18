---
locale: en-US
source_revision: c63b4582e5ea4e7eac387ce70eb29aed40ab3d4cb5d5c76ae95a6cd8fa1f8f69
api_id: Decimal::scale
---
# Decimal::scale

Read the stored nonnegative decimal scale.

```rust
fn scale (& self) -> usize
```

## Parameters, return and usage notes

Returns usize as an exact number within the playground scale budget. It has the same value as precision().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-scale" locale="en-US" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
