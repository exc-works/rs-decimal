---
locale: en-GB
source_revision: ea763f949f6ea9085a5bdbf656ce6fbf3e8731a7f67b0b4db3858116c77c3d15
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::lt
---
# Decimal::lt

Test whether this value is less than another.

```rust
fn lt (& self , other : & Self) -> bool
```

## Parameters, return and usage notes

Borrow other: &Decimal; return bool/boolean. Equal numerical values return false even if their stored scales differ.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lt" locale="en-GB" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
