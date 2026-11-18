---
locale: en-GB
source_revision: 0ad883afdae2ec6d62af55006e383eb095f5c7928dcdbf6c2f7baf446a49d945
translation_provenance: en-US-reviewed-reuse
api_id: GoFormat.width
---
# GoFormat.width

Set the width format field.

```rust
width: Option < usize >
```

## Parameters, return and normal usage

Minimum width includes sign and decimal point; it pads without truncating longer text. None/null disables minimum width. Core formatting counts output bytes.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-width" locale="en-GB" />

## Browser boundaries

The [resource policy](../resource-policy) caps format width at 65536 bytes and precision at 1000; fixed formatting also checks coefficient growth. Buffers are limited to 65536 bytes and append uses a conservative size estimate, which may reject an operation that would just fit. Resource errors do not mutate inputs or buffers. print separately truncates long display text; inspect length or print smaller slices when examining a full result.

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
