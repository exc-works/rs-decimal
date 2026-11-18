---
locale: en-US
source_revision: 5323ce455eba922d5a5c86c41373b1d3638bbe014511b21ca1e6f036392adab6
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

<MethodExample id="formatting-goformat-alternate" locale="en-US" />

## Browser boundaries

The [resource policy](../resource-policy) caps format width at 65536 bytes and precision at 1000; fixed formatting also checks coefficient growth. Buffers are limited to 65536 bytes and append uses a conservative size estimate, which may reject an operation that would just fit. Resource errors do not mutate inputs or buffers. print separately truncates long display text; inspect length or print smaller slices when examining a full result.

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
