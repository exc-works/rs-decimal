---
locale: en-GB
source_revision: 95811018cff564b633ac5b58792bd265ffa12dcefa76e1d1dd9c8c151efed3fd
translation_provenance: en-US-reviewed-reuse
api_id: GoFormat::impl Copy
---
# GoFormat::impl Copy

Use the Rust Copy implementation.

```rust
impl Copy for GoFormat
```

## Parameters, return and normal usage

GoFormat is a small Copy field struct. Clone/Copy preserve options; Debug prints fields. Default selects v, None width/precision and all flags false. This Rust syntax is tested natively; JavaScript objects use normal JS field-object semantics and GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-copy" locale="en-GB" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
