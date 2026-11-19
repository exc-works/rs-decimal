---
locale: en-GB
source_revision: eb4bab6a9fdfe771d31eefc8731c88dad81f2f8a3273adea4040fea11f4141ab
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::sign
---
# Decimal::sign

Read the sign as -1, 0, or 1.

```rust
fn sign (& self) -> i32
```

## Parameters, return and usage notes

Returns i32/number. Both initialized and uninitialized zero have sign 0; there is no separate negative-zero sign.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-sign" locale="en-GB" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
