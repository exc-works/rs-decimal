---
locale: en-GB
source_revision: 1bcdb4a809c3db807556fea8772e6afac8daed8fef0faa997fbb848ae1a34a13
translation_provenance: en-US-reviewed-reuse
api_id: GoFormat.left
---
# GoFormat.left

Set the left format field.

```rust
left: bool
```

## Parameters, return and normal usage

Pad on the right with spaces. Left alignment overrides numeric zero padding. Default false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-left" locale="en-GB" />

## Browser boundaries

The [resource policy](../resource-policy) caps format width at 65536 bytes and precision at 1000; fixed formatting also checks coefficient growth. Buffers are limited to 65536 bytes and append uses a conservative size estimate, which may reject an operation that would just fit. Resource errors do not mutate inputs or buffers. print separately truncates long display text; inspect length or print smaller slices when examining a full result.

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
