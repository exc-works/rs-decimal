---
locale: en-US
source_revision: c98261b50ba3febe344cea19fd963c1739045fe7ff1872b278b97c36e7fef20b
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

<MethodExample id="formatting-decimal-format-with-separators" locale="en-US" />

## Browser boundaries

The [resource policy](../resource-policy) caps format width at 65536 bytes and precision at 1000; fixed formatting also checks coefficient growth. Buffers are limited to 65536 bytes and append uses a conservative size estimate, which may reject an operation that would just fit. Resource errors do not mutate inputs or buffers. print separately truncates long display text; inspect length or print smaller slices when examining a full result.

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
