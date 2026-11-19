---
locale: en-US
source_revision: c7a338ce433accfeb61051f993009577dfae7589a629a17bae6043234f27ca38
api_id: Decimal::max
---
# Decimal::max

Copy the larger of two numerical values.

```rust
fn max (& self , other : & Self) -> Self
```

## Parameters, return and usage notes

Borrow other: &Decimal; return an independent Decimal. A tie preserves the first operand’s scale and initialization state. Use Decimal::max(&left, &right) in Rust to select this borrowed inherent method explicitly; Ord::max takes ownership.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-max" locale="en-US" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
