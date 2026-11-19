---
locale: en-GB
source_revision: 56094af9f622985dec933077af65cd0cc673023dc5a3a4be3553f2af47b621d4
translation_provenance: en-US-reviewed-reuse
api_id: Decimal
---
# Decimal

Store an exact coefficient and a nonnegative decimal scale.

```rust
pub struct Decimal
```

## Normal usage and semantics

The value is coefficient × 10^-scale. Scale counts decimal places, including retained trailing zeros. Default is an uninitialized numerical zero. Constructors create initialized values; borrowed methods preserve inputs, while consuming methods invalidate their browser wrappers. Use named constructors in JavaScript; new Decimal() is not a public constructor.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal" locale="en-GB" />

The shared example runs in native Rust and the browser.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
