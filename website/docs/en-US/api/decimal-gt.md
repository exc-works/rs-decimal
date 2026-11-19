---
locale: en-US
source_revision: f3c9953fc7c2ab0c097e714a9d8217e98239ea7e625d7a02d8c88afee82038af
api_id: Decimal::gt
---
# Decimal::gt

Test whether this value is greater than another.

```rust
fn gt (& self , other : & Self) -> bool
```

## Parameters, return and usage notes

Borrow other: &Decimal; return bool/boolean. Equal numerical values return false even if their stored scales differ.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-gt" locale="en-US" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
