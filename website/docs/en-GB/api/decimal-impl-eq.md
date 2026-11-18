---
locale: en-GB
source_revision: cfbab45ed5115c58df815ab40afe368434f97862eeaecc53b795f1f752fe2e25
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::impl Eq
---
# Decimal::impl Eq

Use Decimal where Rust requires total equality.

```rust
impl Eq for Decimal
```

## Normal usage and semantics

Eq is a marker trait with no methods. Equality compares numbers and ignores scale or initialization. It does not imply JavaScript object identity equality.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-eq" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
