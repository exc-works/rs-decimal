---
locale: en-US
source_revision: 9a05736a78d12045af2704500028fdc4fdec0490c797ca6d8ba9130b66fbc153
api_id: min
---
# min

Copy the smaller of two decimal values using a free function.

```rust
fn min (a : & Decimal , b : & Decimal) -> Decimal
```

## Parameters, return and usage notes

Borrow a and b: &Decimal; return an independent Decimal. A numerical tie preserves a’s scale and initialization state. This calls the same library semantics as Decimal::min.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-min" locale="en-US" />

## Boundaries

The bridge preserves Rust names and parameter order. Free functions are available directly in the playground. Borrowed inputs are not consumed or changed; the bridge rejects invalid or foreign-run handles. All operations follow the [resource policy](../resource-policy).

## Related methods

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
