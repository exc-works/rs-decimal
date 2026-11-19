---
locale: en-GB
source_revision: 94e5c200ef4c48b600cdc1b46dd606d6ca8e9e2741aedbb2bac50f53ac53727e
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::new
---
# Decimal::new

Create an initialized whole number. Use this constructor for a signed 64-bit integer such as a count or an integral amount.

```rust
fn new (value : i64) -> Self
```

## Parameters and return

`value: i64` → JavaScript `bigint`. The result has scale zero.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new" locale="en-GB" />

## Usage notes

Use `12n`, not a JavaScript number. Values outside i64 are rejected in JavaScript; decimal fractions belong in `new_from_string`.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
