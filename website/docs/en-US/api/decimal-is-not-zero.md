---
locale: en-US
source_revision: 19e13f576512aa6ef6c12c54900338d87cb485da07e25d15d575e4c5353b491a
api_id: Decimal::is_not_zero
---
# Decimal::is_not_zero

Test whether the numerical value differs from zero.

```rust
fn is_not_zero (& self) -> bool
```

## Parameters, return and usage notes

Returns bool/boolean, the inverse of is_zero(). An uninitialized default returns false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-not-zero" locale="en-US" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
