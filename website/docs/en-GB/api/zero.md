---
locale: en-GB
source_revision: 68f642c8ee77ff168dce0e11e2443279259a34decdc0113ce303a8ad64b12c0c
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

## Related methods

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
