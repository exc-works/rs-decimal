---
locale: en-US
source_revision: 6a03ad3f2e6f0755e57eb89608d0891566bd0e9c4b875afd8935ad4e0e27dd6d
api_id: GoFormat::impl Default::default
---
# GoFormat::impl Default::default

Use the Rust Default::default implementation.

```rust
fn default () -> Self
```

## Parameters, return and normal usage

GoFormat is a small Copy field struct. Clone/Copy preserve options; Debug prints fields. Default selects v, None width/precision and all flags false. This Rust syntax is tested natively; JavaScript objects use normal JS field-object semantics and GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-default-default" locale="en-US" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
