---
locale: en-GB
source_revision: 6d83b155d9084cec9f42deea0ac618a5403b16dd0003e0789794c0428f3a019a
translation_provenance: en-US-reviewed-reuse
api_id: NullDecimal::new
---
# NullDecimal::new

Move a decimal into a present nullable value.

```rust
fn new (decimal : Decimal) -> Self
```

## Normal usage and semantics

decimal: Decimal is consumed; the result always has valid=true, even if decimal is uninitialized zero. The moved JavaScript input can no longer be used. Use a clone when the original must remain available.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-new" locale="en-GB" />

The shared example runs in native Rust and the browser.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
