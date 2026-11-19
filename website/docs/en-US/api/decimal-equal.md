---
locale: en-US
source_revision: 05d4c976c4f23a3286f06a831a12b99e1cda6e2ba4014722196a6ec9aade2d22
api_id: Decimal::equal
---
# Decimal::equal

Test whether two decimals represent the same number.

```rust
fn equal (& self , other : & Self) -> bool
```

## Parameters, return and usage notes

Borrow other: &Decimal; return bool/boolean. Scale and initialization state are ignored. JavaScript === compares wrapper identity; call equal() for Rust numerical equality.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-equal" locale="en-US" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
