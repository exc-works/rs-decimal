---
locale: en-GB
source_revision: 84f61035965dcc9b605c00e7c877afbafcb7870fb2b3b89e0902348c80e03dd7
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::string
---
# Decimal::string

Format fixed-point decimal text.

```rust
fn string (& self) -> String
```

## Parameters, return and normal usage

No parameters; returns String/string without scientific notation. Removes redundant fractional trailing zeros; integer zeros remain. Uses a dot and no grouping. Negative zero has no separate representation. Default zero prints 0 without becoming initialized; formatting borrows the input.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-string" locale="en-GB" />

## Browser boundaries

The [resource policy](../resource-policy) caps format width at 65536 bytes and precision at 1000; fixed formatting also checks coefficient growth. Buffers are limited to 65536 bytes and append uses a conservative size estimate, which may reject an operation that would just fit. Resource errors do not mutate inputs or buffers. print separately truncates long display text; inspect length or print smaller slices when examining a full result.

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
