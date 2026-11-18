---
locale: en-US
source_revision: aa764a04d58bbac840fe64bca919cb70175f064a9010f9a840a1f56ffacabc1c
api_id: Decimal::is_zero
---
# Decimal::is_zero

Test numerical zero before a value-dependent operation.

```rust
fn is_zero (& self) -> bool
```

## Parameters, return and usage notes

Returns bool/boolean and also returns true for the uninitialized default. Use is_nil() when initialization state matters.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-zero" locale="en-US" />

## Errors and boundaries

These methods borrow inputs without consuming or changing them. The library operation does not return Result. The bridge rejects disposed, moved or foreign-run handles and applies the [resource policy](../resource-policy). Examples wrap scalar output in a `value` field; Decimal output uses its complete snapshot.

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
