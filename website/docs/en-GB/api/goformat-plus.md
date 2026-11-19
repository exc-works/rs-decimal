---
locale: en-GB
source_revision: 0989f60fe84cf7be4e7708a7140f9cc16468c21ad57d855b67f896084ddd8606
translation_provenance: en-US-reviewed-reuse
api_id: GoFormat.plus
---
# GoFormat.plus

Set the plus format field.

```rust
plus: bool
```

## Parameters, return and normal usage

Show + for nonnegative numeric output. Takes precedence over space. Applies to numeric verbs, not v/s/q; negative values retain -. Default false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-plus" locale="en-GB" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
