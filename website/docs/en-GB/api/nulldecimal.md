---
locale: en-GB
source_revision: 8675e68949343d33893aa6f81cc6a57f32cdafd3aaf80161520178113f46a31d
translation_provenance: en-US-reviewed-reuse
api_id: NullDecimal
---
# NullDecimal

Represent an optional decimal with independent validity and stored value.

```rust
pub struct NullDecimal
```

## Normal usage and semantics

valid alone controls presence. An invalid nullable may retain a nonzero decimal; a valid nullable may contain uninitialized zero. The public Rust decimal field is exposed through explicit copying/moving adapters in JavaScript. Default is absent.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal" locale="en-GB" />

The shared example runs in native Rust and the browser.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
