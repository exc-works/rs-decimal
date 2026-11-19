---
locale: en-US
source_revision: df43bbe7e732dc8914c18a95d0b5d68a3839eb4a2b1c83d4dc811e0031827a91
api_id: GoFormat::impl Clone
---
# GoFormat::impl Clone

Use the Rust Clone implementation.

```rust
impl Clone for GoFormat
```

## Parameters, return and normal usage

GoFormat is a small Copy field struct. Clone/Copy preserve options; Debug prints fields. Default selects v, None width/precision and all flags false. This Rust syntax is tested natively; bridge objects use normal JS field-object semantics and GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-clone" locale="en-US" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
