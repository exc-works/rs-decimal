---
locale: en-US
source_revision: 07a3f028e4aca5abdb58974bbb6c24c9b9018814af668c73f9dc49c66b12687d
api_id: GoFormat::impl Default
---
# GoFormat::impl Default

Use the Rust Default implementation.

```rust
impl Default for GoFormat
```

## Parameters, return and normal usage

GoFormat is a small Copy field struct. Clone/Copy preserve options; Debug prints fields. Default selects v, None width/precision and all flags false. This Rust syntax is tested natively; JavaScript objects use normal JS field-object semantics and GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-default" locale="en-US" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
