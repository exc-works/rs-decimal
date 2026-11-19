---
locale: en-US
source_revision: 5f1761508e55494581ac3ef80177b3b3480f53a919c2efbe231b66c02ccf3a14
api_id: GoFormat.precision
---
# GoFormat.precision

Set the precision format field.

```rust
precision: Option < usize >
```

## Parameters, return and normal usage

Verb-specific precision: f/F use fractional places, e/E use places after the leading digit, g/G use significant digits (zero treated as one), and d uses minimum integer digits. None/null keeps verb-specific defaults; f retains scale, e uses six places.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-precision" locale="en-US" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
