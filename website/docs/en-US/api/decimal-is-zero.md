---
locale: en-US
source_revision: 3a7d890770468c269032ca95ee3365a1f5fdeb70051590bf674c733d1e1658d6
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

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
