---
locale: en-GB
source_revision: 65703feb0e11af9dfe348c058c514dbf10ab13c1d2a720632926e57fee628fa9
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::impl FromStr
---
# Decimal::impl FromStr

Parse decimal text using Rust parse and FromStr.

```rust
impl FromStr for Decimal
```

## Normal usage and semantics

Returns Decimal or DecimalError. Syntax and zero normalization are the same as new_from_string. The browser calls that named method; Rust trait syntax is native-only.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr" locale="en-GB" />

This Rust syntax example runs in native tests and does not claim browser trait support.

## Related documentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
