---
locale: en-US
source_revision: 46f70cc4972a4c4ca833f0191402c07a03bbeeb19c10706e521c3bfe03276dbf
api_id: one
---
# one

Construct the decimal integer one.

```rust
fn one () -> Decimal
```

## Parameters, return and usage notes

No parameters; returns a fresh initialized Decimal at scale zero. Mutating or freeing one result does not affect later calls.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-one" locale="en-US" />

## Related methods

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
