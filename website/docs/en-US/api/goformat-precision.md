---
locale: en-US
source_revision: b5c0056c42f3f4e6d3613af84d12321baa6359b54407d77f1aacdb7d5d883dd7
api_id: GoFormat.precision
---
# GoFormat.precision

Set the precision format field.

```rust
precision: Option < usize >
```

## Parameters, return and normal usage

Verb-specific precision: f/F use fractional places, e/E use places after the leading digit, g/G use significant digits (zero treated as one), and d uses minimum integer digits. None/null keeps verb-specific defaults; f retains scale, e uses six places.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-precision" locale="en-US" />

## Browser boundaries

The [resource policy](../resource-policy) caps format width at 65536 bytes and precision at 1000; fixed formatting also checks coefficient growth. Buffers are limited to 65536 bytes and append uses a conservative size estimate, which may reject an operation that would just fit. Resource errors do not mutate inputs or buffers. print separately truncates long display text; inspect length or print smaller slices when examining a full result.

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
