---
locale: en-US
source_revision: d57c1e8ddb07024ed3c5c002f095b2cee086c1a0719df70a0b4ba8761f29418d
api_id: NullDecimal.valid
---
# NullDecimal.valid

Read or update presence without changing the stored decimal.

```rust
valid: bool
```

## Normal usage and semantics

This public Rust bool field maps to a boolean getter/setter. Setting false does not erase the decimal; setting true does not initialize it. The bridge rejects non-boolean values before mutation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-valid" locale="en-US" />

The shared example runs in native Rust and the browser.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
