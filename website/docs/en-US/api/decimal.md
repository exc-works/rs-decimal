---
locale: en-US
source_revision: 9459cb9d77c7f6ad22b96853baef01b1a9461529cecff18a116b5d2dee9166d3
api_id: Decimal
---
# Decimal

Store an exact coefficient and a nonnegative decimal scale.

```rust
pub struct Decimal
```

## Normal usage and semantics

The value is coefficient × 10^-scale. Scale counts decimal places, including retained trailing zeros. Default is an uninitialized numerical zero. Constructors create initialized values; borrowed methods preserve inputs, while consuming methods invalidate their browser wrappers. Use named constructors on the bridge; new Decimal() is not a public constructor.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal" locale="en-US" />

The shared example runs in native Rust and the browser.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
