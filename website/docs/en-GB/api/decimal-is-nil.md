---
locale: en-GB
source_revision: 59688cddd572412f36dd0f36cc15c34efbc891dab38cd57b614763347d469369
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::is_nil
---
# Decimal::is_nil

Check whether a decimal is the uninitialized default value.

```rust
fn is_nil (& self) -> bool
```

## Parameters, return and usage notes

Returns bool/boolean. Default is nil, while constructed zero is initialized; both compare numerically equal. This is not the validity flag of NullDecimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-nil" locale="en-GB" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
