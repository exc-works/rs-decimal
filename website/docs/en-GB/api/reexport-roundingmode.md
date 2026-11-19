---
locale: en-GB
source_revision: 73057fa1830878a540ff385b7411b32617a1d65647f7c283e5495a0cce3aca84
translation_provenance: en-US-reviewed-reuse
api_id: reexport::RoundingMode
---
# reexport::RoundingMode

Import rounding modes from the crate root.

```rust
pub use rounding::RoundingMode as RoundingMode
```

## Parameters, return and normal usage

Use rs_decimal::RoundingMode. This is the same enum, not a second implementation. The browser exposes the named RoundingMode object without Rust import syntax.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-reexport-roundingmode" locale="en-GB" />

## Related methods

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
