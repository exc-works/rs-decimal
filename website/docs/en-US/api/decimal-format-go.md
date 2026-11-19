---
locale: en-US
source_revision: f9b1eaa0c6a10e9f32908baa1f2c09a9aafeb3265bffa095bb0a0b210e1e1f05
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

<MethodExample id="formatting-decimal-format-go" locale="en-US" />

## Related methods

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
