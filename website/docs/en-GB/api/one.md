---
locale: en-GB
source_revision: 939b86ba367c0b35cb69b9f2664eb74fafe4279f8b1b81c20315cc4e07b18382
translation_provenance: en-US-reviewed-reuse
api_id: one
---
# one

Construct the decimal integer one.

```rust
fn one () -> Decimal
```

## Parameters, return and usage notes

No parameters; returns a fresh initialized Decimal at scale zero. Mutating or freeing one result does not affect later calls.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-one" locale="en-GB" />

## Boundaries

The bridge preserves Rust names and parameter order. Free functions are available directly in the playground. Borrowed inputs are not consumed or changed; the bridge rejects invalid or foreign-run handles. All operations follow the [resource policy](../resource-policy).

## Related methods

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
