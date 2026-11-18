---
locale: en-GB
source_revision: b8d0350ab2d3257ebc2822de61360d2033648c92799436b97481a4c0e75947bf
translation_provenance: en-US-reviewed-reuse
api_id: GoFormat.plus
---
# GoFormat.plus

Set the plus format field.

```rust
plus: bool
```

## Parameters, return and normal usage

Show + for nonnegative numeric output. Takes precedence over space. Applies to numeric verbs, not v/s/q; negative values retain -. Default false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-plus" locale="en-GB" />

## Browser boundaries

The [resource policy](../resource-policy) caps format width at 65536 bytes and precision at 1000; fixed formatting also checks coefficient growth. Buffers are limited to 65536 bytes and append uses a conservative size estimate, which may reject an operation that would just fit. Resource errors do not mutate inputs or buffers. print separately truncates long display text; inspect length or print smaller slices when examining a full result.

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
