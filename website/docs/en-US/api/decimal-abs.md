---
locale: en-US
source_revision: c71c6b4ce225ea223944d584a5ec03b1adfe1dbd2adeb04f6beb38800f1e252e
api_id: Decimal::abs
---
# Decimal::abs

Create an independent decimal containing the absolute value.

```rust
fn abs (& self) -> Self
```

## Parameters, return and usage notes

Returns Decimal and preserves scale and trailing zeros. The result is initialized even when the input is default; the input is unchanged.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-abs" locale="en-US" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
