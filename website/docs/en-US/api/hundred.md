---
locale: en-US
source_revision: 075546f79884d636c4e552add1629511a5f2dca740186a5700262f084c5bd8a5
api_id: hundred
---
# hundred

Construct the decimal integer one hundred.

```rust
fn hundred () -> Decimal
```

## Parameters, return and usage notes

No parameters; returns a fresh initialized Decimal at scale zero. It is a value constructor, not a shared mutable constant.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-hundred" locale="en-US" />

## Boundaries

The bridge preserves Rust names and parameter order. Free functions are available directly in the playground. Borrowed inputs are not consumed or changed; the bridge rejects invalid or foreign-run handles. All operations follow the [resource policy](../resource-policy).

## Related methods

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
