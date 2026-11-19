---
locale: en-US
source_revision: 84828af3d16c67c7fecd4ce94d987a7814ff2080c1c08e3527725c6cb1588c42
api_id: Decimal::neg
---
# Decimal::neg

Create an independent decimal with the sign reversed.

```rust
fn neg (& self) -> Self
```

## Parameters, return and usage notes

Returns Decimal and preserves scale and trailing zeros. Zero stays numerically zero; the result is initialized even for a default input.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-neg" locale="en-US" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
