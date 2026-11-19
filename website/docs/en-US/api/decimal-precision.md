---
locale: en-US
source_revision: 0ad97f33b356cd281b88951f90d8d3c10dff06f728f22f7136b85c34bc769e4e
api_id: Decimal::precision
---
# Decimal::precision

Read the number of decimal places stored by a value.

```rust
fn precision (& self) -> isize
```

## Parameters, return and usage notes

Returns an isize (integer number in the browser). This is the scale, not the number of significant digits. Trailing zeros contribute to it.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-precision" locale="en-US" />

## Related methods

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
