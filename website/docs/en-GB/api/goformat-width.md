---
locale: en-GB
source_revision: fe33f9fd8631eeeae5a499a47abbc262340fc190757785abe0749a14c97bf791
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

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
