---
locale: en-GB
source_revision: e9b605c9c2b762d25f61c081bee35aa1c375be0cb3cab1e12e2bd0938c7a3c70
translation_provenance: en-US-reviewed-reuse
api_id: GoFormat
---
# GoFormat

Describe a single formatting operation with a field object.

```rust
pub struct GoFormat
```

## Parameters, return and normal usage

Fields mirror the Rust struct: verb, width, precision, left, plus, space, zero and alternate. GoFormat.default() in the bridge reads Rust defaults and returns a fresh mutable object; changing it does not set global formatting. width/precision use null for None and nonnegative wasm32 integer numbers for Some. Rust traits remain native-only syntax.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat" locale="en-GB" />

## Browser boundaries

The [resource policy](../resource-policy) caps format width at 65536 bytes and precision at 1000; fixed formatting also checks coefficient growth. Buffers are limited to 65536 bytes and append uses a conservative size estimate, which may reject an operation that would just fit. Resource errors do not mutate inputs or buffers. print separately truncates long display text; inspect length or print smaller slices when examining a full result.

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
