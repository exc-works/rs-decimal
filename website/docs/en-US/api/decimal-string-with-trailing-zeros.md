---
locale: en-US
source_revision: 7fbcc33cd9efa5ed33f322356ce37cb91f352f88d4c3bb3845c8245fa3ded94d
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

## Browser boundaries

The [resource policy](../resource-policy) caps format width at 65536 bytes and precision at 1000; fixed formatting also checks coefficient growth. Buffers are limited to 65536 bytes and append uses a conservative size estimate, which may reject an operation that would just fit. Resource errors do not mutate inputs or buffers. print separately truncates long display text; inspect length or print smaller slices when examining a full result.

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
