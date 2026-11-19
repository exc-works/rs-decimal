---
locale: en-GB
source_revision: 88974eef4784bf8f1c8b6ec4ce25d3380a2169d449467ecdb4cee7997d0acdd8
translation_provenance: en-US-reviewed-reuse
api_id: GoFormat::impl Clone
---
# GoFormat::impl Clone

Use the Rust Clone implementation.

```rust
impl Clone for GoFormat
```

## Parameters, return and normal usage

GoFormat is a small Copy field struct. Clone/Copy preserve options; Debug prints fields. Default selects v, None width/precision and all flags false. This Rust syntax is tested natively; JavaScript objects use normal JS field-object semantics and GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-clone" locale="en-GB" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
