---
locale: en-US
source_revision: 0d4a74db932ab126e7566fb630352abba8c528142a3d37c615a2694b7fac9c0a
api_id: max
---
# max

Copy the larger of two decimal values using a free function.

```rust
fn max (a : & Decimal , b : & Decimal) -> Decimal
```

## Parameters, return and usage notes

Borrow a and b: &Decimal; return an independent Decimal. A numerical tie preserves a’s scale and initialization state. This calls the same library semantics as Decimal::max.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-max" locale="en-US" />

## Boundaries

The bridge preserves Rust names and parameter order. Free functions are available directly in the playground. Borrowed inputs are not consumed or changed; the bridge rejects invalid or foreign-run handles. All operations follow the [resource policy](../resource-policy).

## Related methods

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
