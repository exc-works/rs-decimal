---
locale: en-US
source_revision: f320c1a30df05777794b5d83edd6494fed5ebfe3f6b80e1d48ebc83fa5e17748
api_id: Decimal::string_with_trailing_zeros
---
# Decimal::string_with_trailing_zeros

Format fixed-point decimal text.

```rust
fn string_with_trailing_zeros (& self) -> String
```

## Parameters, return and normal usage

No parameters; returns String/string without scientific notation. Retains every stored decimal place, including trailing zeros. Uses a dot and no grouping. Negative zero has no separate representation. Default zero prints 0 without becoming initialized; formatting borrows the input.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-string-with-trailing-zeros" locale="en-US" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
