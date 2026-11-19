---
locale: en-GB
source_revision: 711eb90b21b1b79f0303fceb3e98f79a6b9b78a5eb632f30a661f662027c1167
translation_provenance: en-US-reviewed-reuse
api_id: GoFormat.alternate
---
# GoFormat.alternate

Set the alternate format field.

```rust
alternate: bool
```

## Parameters, return and normal usage

Keep a decimal point for f/F/e/E and retain general-format trailing zeros for g/G. Does not mean alternate numeric base. Default false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-alternate" locale="en-GB" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
