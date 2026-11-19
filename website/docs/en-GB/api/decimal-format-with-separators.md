---
locale: en-GB
source_revision: c3fbe0927c540d91118929ccf7291b490ab739216719c7655f9f81744f8edff3
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::format_with_separators
---
# Decimal::format_with_separators

Group integer digits and choose the decimal separator.

```rust
fn format_with_separators (& self , thousands : char , decimal : char) -> String
```

## Parameters, return and normal usage

thousands: char; decimal: char. Returns String/string with groups of three integer digits and every stored fractional place. Each JS argument must be exactly one Unicode scalar, including supplementary characters; lone surrogates and multi-character strings are rejected. NUL thousands disables grouping; NUL decimal selects a dot. Separators are literal, not locale names.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-with-separators" locale="en-GB" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
