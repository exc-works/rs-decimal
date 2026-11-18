---
locale: en-GB
source_revision: 6ee970e3e2507cdd225eb4c314175b5a95b9aa45ac2e64ed4be5fded0c11b425
translation_provenance: en-US-reviewed-reuse
api_id: GoFormat.space
---
# GoFormat.space

Set the space format field.

```rust
space: bool
```

## Parameters, return and normal usage

Add a leading space for nonnegative numeric output when plus is false. Negative values retain their sign. Default false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-space" locale="en-GB" />

## Browser boundaries

The [resource policy](../resource-policy) caps format width at 65536 bytes and precision at 1000; fixed formatting also checks coefficient growth. Buffers are limited to 65536 bytes and append uses a conservative size estimate, which may reject an operation that would just fit. Resource errors do not mutate inputs or buffers. print separately truncates long display text; inspect length or print smaller slices when examining a full result.

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
