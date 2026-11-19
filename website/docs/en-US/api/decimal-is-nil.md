---
locale: en-US
source_revision: 7d0c030a450a67fa4b0e618dbc6cb501daeb5fbc64647ae39a02ca84be312b44
api_id: Decimal::is_nil
---
# Decimal::is_nil

Check whether a decimal is the uninitialized default value.

```rust
fn is_nil (& self) -> bool
```

## Parameters, return and usage notes

Returns bool/boolean. Default is nil, while constructed zero is initialized; both compare numerically equal. This is not the validity flag of NullDecimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-nil" locale="en-US" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
