---
locale: en-US
source_revision: 5fb7d2ba1d450e55c06f4bd243bcd776a0cb6c5e8f2e0f80f16235d6abf720a7
api_id: GoFormat.zero
---
# GoFormat.zero

Set the zero format field.

```rust
zero: bool
```

## Parameters, return and normal usage

Fill numeric left padding with zeros after the sign. Ignored when left is true or d has explicit precision. Default false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-zero" locale="en-US" />

## Browser boundaries

The [resource policy](../resource-policy) caps format width at 65536 bytes and precision at 1000; fixed formatting also checks coefficient growth. Buffers are limited to 65536 bytes and append uses a conservative size estimate, which may reject an operation that would just fit. Resource errors do not mutate inputs or buffers. print separately truncates long display text; inspect length or print smaller slices when examining a full result.

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
