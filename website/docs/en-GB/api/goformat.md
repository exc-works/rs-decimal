---
locale: en-GB
source_revision: e057a9c108e2aa51ba03c94655793b13aece7049e3b5085cd912fedc0ed1d203
translation_provenance: en-US-reviewed-reuse
api_id: GoFormat
---
# GoFormat

Describe a single formatting operation with a field object.

```rust
pub struct GoFormat
```

## Parameters, return and normal usage

Fields mirror the Rust struct: verb, width, precision, left, plus, space, zero and alternate. GoFormat.default() in JavaScript reads Rust defaults and returns a fresh mutable object; changing it does not set global formatting. width/precision use null for None and nonnegative wasm32 integer numbers for Some. Rust traits remain native-only syntax.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat" locale="en-GB" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
