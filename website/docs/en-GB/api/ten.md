---
locale: en-GB
source_revision: df5bdc8fba1feff0933958fdceab2cd0bf8d1a541a0f2e47f17bffe6a6338d47
translation_provenance: en-US-reviewed-reuse
api_id: ten
---
# ten

Construct the decimal integer ten.

```rust
fn ten () -> Decimal
```

## Parameters, return and usage notes

No parameters; returns a fresh initialized Decimal at scale zero. Use rescale when decimal places are required.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-ten" locale="en-GB" />

## Boundaries

The bridge preserves Rust names and parameter order. Free functions are available directly in the playground. Borrowed inputs are not consumed or changed; the bridge rejects invalid or foreign-run handles. All operations follow the [resource policy](../resource-policy).

## Related methods

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
