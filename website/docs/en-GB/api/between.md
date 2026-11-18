---
locale: en-GB
source_revision: 4488da41f235921047e8f85b8a68b47bc497bf199529326773333d8f22d38585
translation_provenance: en-US-reviewed-reuse
api_id: between
---
# between

Check whether a decimal lies within an inclusive numerical range.

```rust
fn between (v : & Decimal , lower : & Decimal , upper : & Decimal) -> bool
```

## Parameters, return and usage notes

Borrow v, lower and upper: &Decimal; return bool/boolean for lower <= v <= upper. Equality ignores representation. Bounds are not reordered: lower > upper produces false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-between" locale="en-GB" />

## Boundaries

The bridge preserves Rust names and parameter order. Free functions are available directly in the playground. Borrowed inputs are not consumed or changed; the bridge rejects invalid or foreign-run handles. All operations follow the [resource policy](../resource-policy).

## Related methods

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
