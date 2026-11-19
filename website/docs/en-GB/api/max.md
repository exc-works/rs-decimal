---
locale: en-GB
source_revision: f49d2dc4c074cf4f52b2a3a197eb1d13f1144942ab8a1df7ea4bf51caaaa350f
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="helper-max" locale="en-GB" />

## Related methods

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
