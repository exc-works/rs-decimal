---
locale: en-GB
source_revision: 57e2d57e143cb2ba5e54935f1231d3095826fead303cbd95c70c73feba2c6bfd
translation_provenance: en-US-reviewed-reuse
api_id: hundred
---
# hundred

Construct the decimal integer one hundred.

```rust
fn hundred () -> Decimal
```

## Parameters, return and usage notes

No parameters; returns a fresh initialized Decimal at scale zero. It is a value constructor, not a shared mutable constant.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-hundred" locale="en-GB" />

## Related methods

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
