---
locale: en-US
source_revision: 2a36a5c46f57052b68644728359c44f15465f6bfca2c60d1540e40c603cc8c5b
api_id: NullDecimal.valid
---
# NullDecimal.valid

Read or update presence without changing the stored decimal.

```rust
valid: bool
```

## Normal usage and semantics

This public Rust bool field maps to a boolean getter/setter. Setting false does not erase the decimal; setting true does not initialize it. The JavaScript API rejects non-boolean values before mutation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-valid" locale="en-US" />

The shared example runs in native Rust and the browser.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
