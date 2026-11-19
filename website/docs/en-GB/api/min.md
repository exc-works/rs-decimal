---
locale: en-GB
source_revision: 9f58b0f5c6b478fabd100e00297f6dfcdae4ff2a32ead701d8b208c279651c42
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="helper-min" locale="en-GB" />

## Related methods

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
