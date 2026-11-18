---
locale: en-US
source_revision: 74688d1ee73b78a25c1792ba73a6a2e013084b0279376cef739a1b5c8cbed64f
api_id: NullDecimal.decimal
---
# NullDecimal.decimal

Inspect or replace the stored decimal independently of validity.

```rust
decimal: Decimal
```

## Normal usage and semantics

Rust exposes a mutable Decimal field. The browser uses get_decimal_clone() for an independent copy and set_decimal(value) to consume a replacement. Neither adapter changes valid. These explicit adapters avoid promising a live Rust reference in JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-decimal" locale="en-US" />

The shared example runs in native Rust and the browser.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
