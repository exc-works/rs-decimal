---
locale: en-GB
source_revision: 0d692c90930959d30189d9870456cce3ed78a046b51b42c8004981e031486270
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::format_go
---
# Decimal::format_go

Apply one Go-compatible verb and formatting options.

```rust
fn format_go (& self , f : GoFormat) -> Result < String , DecimalError >
```

## Parameters, return and normal usage

f: GoFormat; returns Result<String, DecimalError>/string. This accepts one verb, not a printf pattern such as %.2f. Supported verbs are v/s/q/d/f/F/e/E/g/G/b. Rounding uses half-even. Unsupported verbs and nonintegral d produce a %! marker as text, not an exception. Fixed precision beyond the native signed range returns InvalidPrecision; browser width/precision/allocation limits are separate. Omitted JS fields use Rust defaults; unknown fields and invalid field types are rejected.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-go" locale="en-GB" />

## Browser boundaries

The [resource policy](../resource-policy) caps format width at 65536 bytes and precision at 1000; fixed formatting also checks coefficient growth. Buffers are limited to 65536 bytes and append uses a conservative size estimate, which may reject an operation that would just fit. Resource errors do not mutate inputs or buffers. print separately truncates long display text; inspect length or print smaller slices when examining a full result.

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
