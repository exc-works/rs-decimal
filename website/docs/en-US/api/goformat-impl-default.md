---
locale: en-US
source_revision: 9f9019ab26936086c52db8997b79603161a6ca91ecf3ebbb2e2fc9ec02fd1a05
api_id: GoFormat::impl Default
---
# GoFormat::impl Default

Use the Rust Default implementation.

```rust
impl Default for GoFormat 
```

## Parameters, return and normal usage

GoFormat is a small Copy field struct. Clone/Copy preserve options; Debug prints fields. Default selects v, None width/precision and all flags false. This Rust syntax is tested natively; bridge objects use normal JS field-object semantics and GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-default" locale="en-US" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
