---
locale: en-GB
source_revision: 751751d0b535375fecff33d2510df43e92ffca892248b1be4dba2a34ce1d0f99
translation_provenance: en-US-reviewed-reuse
api_id: zero
---
# zero

Construct an initialized zero at scale zero.

```rust
fn zero () -> Decimal
```

## Parameters, return and usage notes

No parameters; returns a fresh Decimal. Unlike Decimal::default(), the result is initialized, though both are numerically equal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-zero" locale="en-GB" />

## Boundaries

The bridge preserves Rust names and parameter order. Free functions are available directly in the playground. Borrowed inputs are not consumed or changed; the bridge rejects invalid or foreign-run handles. All operations follow the [resource policy](../resource-policy).

## Related methods

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
