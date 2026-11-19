---
locale: en-US
source_revision: 536eae84e3e9c7c0c702cbcb339f82c64bf0aafab561d4a3f87590939cb52d8e
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

<MethodExample id="helper-between" locale="en-US" />

## Related methods

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
