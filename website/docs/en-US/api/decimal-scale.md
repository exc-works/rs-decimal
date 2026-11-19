---
locale: en-US
source_revision: 7388877c082aab40b6cea1eaae69162f25e5c07d8d3673378f0fdd23fef3e494
api_id: Decimal::scale
---
# Decimal::scale

Read the stored nonnegative decimal scale.

```rust
fn scale (& self) -> usize
```

## Parameters, return and usage notes

Returns usize as an exact number within the playground scale budget. It has the same value as precision().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-scale" locale="en-US" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
