---
locale: en-GB
source_revision: 0c3a3cc5d3969cdd2cb971de3686daa945e18486f55f8c99179deb863727c501
translation_provenance: en-US-reviewed-reuse
api_id: GoFormat.verb
---
# GoFormat.verb

Set the verb format field.

```rust
verb: char
```

## Parameters, return and normal usage

Select one Unicode scalar formatting verb. Default v uses compact fixed-point text; e/E use scientific notation, q quotes, and b prints binary coefficient followed by p-scale (not an IEEE float).

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-verb" locale="en-GB" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
