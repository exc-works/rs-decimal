---
locale: en-US
source_revision: 25693aaef850a3e88dcc638a63de0aa32a65fe1057c9bac969c2400862072704
api_id: Decimal::string
---
# Decimal::string

Format fixed-point decimal text.

```rust
fn string (& self) -> String
```

## Parameters, return and normal usage

No parameters; returns String/string without scientific notation. Removes redundant fractional trailing zeros; integer zeros remain. Uses a dot and no grouping. Negative zero has no separate representation. Default zero prints 0 without becoming initialized; formatting borrows the input.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-string" locale="en-US" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
