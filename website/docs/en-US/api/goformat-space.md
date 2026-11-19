---
locale: en-US
source_revision: c5619d120e2ab677bb25ae58cef9205a1160d82cd87b1081f2a6532750a83303
api_id: GoFormat.space
---
# GoFormat.space

Set the space format field.

```rust
space: bool
```

## Parameters, return and normal usage

Add a leading space for nonnegative numeric output when plus is false. Negative values retain their sign. Default false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-space" locale="en-US" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
