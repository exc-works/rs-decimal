---
locale: en-GB
source_revision: 602dcc7d7a4e6e51eaa040b5b95f43c52285a62967f721872ad5548725d33f50
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::not_equal
---
# Decimal::not_equal

Test whether two decimals represent different numbers.

```rust
fn not_equal (& self , other : & Self) -> bool
```

## Parameters, return and usage notes

Borrow other: &Decimal; return bool/boolean. This is the inverse of equal(), not a comparison of coefficient/scale snapshots.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-not-equal" locale="en-GB" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
